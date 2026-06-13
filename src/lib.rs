//! Library for talking to an 8BitDo Micro controller over Bluetooth Low Energy.
//!
//! The entry point is [`BleKeyboard`]. Connect to the device with
//! [`BleKeyboard::new`], then read or change its key bindings:
//!
//! ```no_run
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! use heigtbitult::BleKeyboard;
//!
//! let mut keyboard = BleKeyboard::new().await?;
//! let bindings = keyboard.read_current_bindings().await?;
//! keyboard.disconnect().await?;
//! # Ok(()) }
//! ```
//!
//! The modules underneath cover the pieces involved:
//!
//! - [`bluetooth`]: finding the adapter, connecting, and reading notifications.
//! - [`bindings`]: the reverse engineered read and write protocol, plus the raw
//!   command bytes.
//! - [`keyboard`]: the [`keyboard::KeyCode`] table mapping HID codes to names.
//! - [`config`]: the device name and the various timeouts.

pub mod bindings;
pub mod bluetooth;
pub mod config;
pub mod keyboard;

use bindings::types::KeyBinding;
use bluetooth::characteristic::get_write_characteristic;
use bluetooth::connect::connect_with_retry;
use bluetooth::power::ensure_bluetooth_enabled;
use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral as BlePeripheral, PeripheralId};
use config::{DEVICE_NAME, SCAN_TIMEOUT};
use futures::stream::{select_all, StreamExt};
use std::collections::HashSet;
use std::error::Error;
use tokio::time::Instant;
use tracing::{debug, error, info};

/// A connected 8BitDo Micro controller.
///
/// Holds the BLE peripheral handle and the characteristic used to read and
/// write bindings. Create one with [`BleKeyboard::new`].
pub struct BleKeyboard {
    peripheral: BlePeripheral,
    characteristic: btleplug::api::Characteristic,
}

impl BleKeyboard {
    /// Finds the controller, connects to it, and gets it ready for use.
    ///
    /// This powers on the Bluetooth adapter if needed (Linux only, via
    /// `bluetoothctl`), scans every adapter for the device advertising as
    /// `80EL`, connects with a few retries, and locates the writable
    /// characteristic.
    ///
    /// Returns an error if there is no adapter, if the device does not appear
    /// within the scan timeout (see [`config::SCAN_TIMEOUT`]), or if the
    /// connection fails.
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        ensure_bluetooth_enabled().await?;
        let manager = Manager::new().await?;
        let adapters = manager.adapters().await?;
        if adapters.is_empty() {
            error!("No Bluetooth adapters found");
            return Err("No Bluetooth adapters found".into());
        }

        let peripheral = scan_for_device(&adapters).await?;

        if !peripheral.is_connected().await? {
            debug!("Device not connected, attempting connection");
            connect_with_retry(&peripheral, 3).await?;
        }

        debug!("Discovering services");
        peripheral.discover_services().await?;
        let characteristic = get_write_characteristic(&peripheral).await?;
        info!("Successfully initialized BLE keyboard");

        Ok(BleKeyboard {
            peripheral,
            characteristic,
        })
    }

    /// Disconnects from the controller.
    pub async fn disconnect(&self) -> Result<(), Box<dyn Error>> {
        self.peripheral.disconnect().await?;
        Ok(())
    }

    /// Reads the 16 key bindings currently stored on the device.
    ///
    /// The result is in button order (see [`config::BUTTON_NAMES`]): index 0 is
    /// `A`, index 15 is `RIGHT`. Each binding is four HID key codes, with `0`
    /// meaning "unused".
    pub async fn read_current_bindings(&mut self) -> Result<Vec<KeyBinding>, Box<dyn Error>> {
        bindings::read::read_current_bindings(&self.peripheral, &self.characteristic).await
    }

    /// Writes a full set of bindings to the device.
    ///
    /// Expects exactly 16 bindings in button order (see
    /// [`config::BUTTON_NAMES`]); anything else is rejected with an error. Each
    /// binding is four HID key codes, with `0` for the slots you do not use.
    pub async fn write_bindings(&mut self, bindings: &[KeyBinding]) -> Result<(), Box<dyn Error>> {
        if bindings.len() != 16 {
            return Err("Expected exactly 16 key bindings".into());
        }
        bindings::write::write_new_bindings(&self.peripheral, &self.characteristic, bindings).await
    }
}

/// Scans every adapter and returns the first peripheral advertising as
/// [`DEVICE_NAME`], or an error if it does not show up within
/// [`SCAN_TIMEOUT`](config::SCAN_TIMEOUT).
///
/// Discovery is event driven: instead of polling the device list on a timer, we
/// subscribe to each adapter's event stream and react the moment the target is
/// advertised. Devices already known to the adapter are checked once up front,
/// and devices that turn out not to match are remembered so their later
/// advertisement updates are ignored.
async fn scan_for_device(adapters: &[Adapter]) -> Result<BlePeripheral, Box<dyn Error>> {
    // Subscribe to events before scanning so we cannot miss an advertisement
    // that lands between starting the scan and reading the stream.
    let mut streams = Vec::with_capacity(adapters.len());
    for (idx, adapter) in adapters.iter().enumerate() {
        let events = adapter.events().await?;
        streams.push(events.map(move |event| (idx, event)));
        debug!("Starting scan on adapter");
        adapter.start_scan(ScanFilter::default()).await?;
    }

    // The device may already be cached from an earlier scan, so check first.
    for adapter in adapters {
        if let Some(peripheral) = cached_target(adapter).await? {
            return stop_and_return(adapters, peripheral).await;
        }
    }

    // Otherwise wait for it to advertise, up to the deadline.
    let deadline = Instant::now() + SCAN_TIMEOUT;
    let mut events = select_all(streams);
    let mut rejected: HashSet<PeripheralId> = HashSet::new();

    loop {
        let next = tokio::time::timeout_at(deadline, events.next()).await;
        let (idx, id) = match next {
            Ok(Some((idx, CentralEvent::DeviceDiscovered(id))))
            | Ok(Some((idx, CentralEvent::DeviceUpdated(id)))) => (idx, id),
            Ok(Some(_)) => continue, // connection/state events we do not care about
            Ok(None) => return Err("Scan event stream ended unexpectedly".into()),
            Err(_) => {
                error!("Device {} not found within {:?}", DEVICE_NAME, SCAN_TIMEOUT);
                return Err("Device not found".into());
            }
        };

        if rejected.contains(&id) {
            continue;
        }

        let peripheral = adapters[idx].peripheral(&id).await?;
        match peripheral.properties().await?.and_then(|p| p.local_name) {
            Some(name) if name == DEVICE_NAME => {
                info!("Found target device: {}", DEVICE_NAME);
                return stop_and_return(adapters, peripheral).await;
            }
            Some(name) => {
                debug!(?name, "Ignoring non-target device");
                rejected.insert(id);
            }
            // No name yet; a later DeviceUpdated may carry it, so do not reject.
            None => {}
        }
    }
}

/// Returns an already-discovered peripheral matching [`DEVICE_NAME`], if any.
async fn cached_target(adapter: &Adapter) -> Result<Option<BlePeripheral>, Box<dyn Error>> {
    for peripheral in adapter.peripherals().await? {
        if let Some(name) = peripheral.properties().await?.and_then(|p| p.local_name)
            && name == DEVICE_NAME
        {
            info!("Found target device: {}", DEVICE_NAME);
            return Ok(Some(peripheral));
        }
    }
    Ok(None)
}

/// Stops scanning on every adapter (best effort) and hands back the peripheral.
async fn stop_and_return(
    adapters: &[Adapter],
    peripheral: BlePeripheral,
) -> Result<BlePeripheral, Box<dyn Error>> {
    for adapter in adapters {
        let _ = adapter.stop_scan().await;
    }
    Ok(peripheral)
}
