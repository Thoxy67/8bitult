// lib.rs
pub mod bindings;
pub mod bluetooth;
pub mod config;
pub mod keyboard;

use bindings::types::KeyBinding;
use bluetooth::characteristic::get_write_characteristic;
use bluetooth::connect::connect_with_retry;
use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::{Manager, Peripheral as BlePeripheral};
use config::DEVICE_NAME;
use std::error::Error;
use tracing::{debug, error, info};

pub struct BleKeyboard {
    peripheral: BlePeripheral,
    characteristic: btleplug::api::Characteristic,
}

impl BleKeyboard {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let manager = Manager::new().await?;
        let adapter_list = manager.adapters().await?;

        if adapter_list.is_empty() {
            error!("No Bluetooth adapters found");
            return Err("No Bluetooth adapters found".into());
        }

        let mut found_device = None;
        for adapter in adapter_list.iter() {
            debug!("Scanning for devices on adapter");
            adapter.start_scan(ScanFilter::default()).await?;
            let peripherals = adapter.peripherals().await?;

            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await?;
                if let Some(local_name) = properties.and_then(|p| p.local_name) {
                    debug!(?local_name, "Found device");
                    if local_name == DEVICE_NAME {
                        info!("Found target device: {}", DEVICE_NAME);
                        found_device = Some(peripheral.clone());
                        break;
                    }
                }
            }
        }

        let peripheral = found_device.ok_or_else(|| {
            error!("Device {} not found", DEVICE_NAME);
            "Device not found"
        })?;

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

    pub async fn disconnect(&self) -> Result<(), Box<dyn Error>> {
        self.peripheral.disconnect().await?;
        Ok(())
    }

    pub async fn read_current_bindings(&mut self) -> Result<Vec<KeyBinding>, Box<dyn Error>> {
        bindings::read::read_current_bindings(&self.peripheral, &self.characteristic).await
    }

    pub async fn write_bindings(&mut self, bindings: &[KeyBinding]) -> Result<(), Box<dyn Error>> {
        if bindings.len() != 16 {
            return Err("Expected exactly 16 key bindings".into());
        }
        bindings::write::write_new_bindings(&self.peripheral, &self.characteristic, bindings).await
    }
}
