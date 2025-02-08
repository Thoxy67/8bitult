pub mod bindings;
pub mod bluetooth;
pub mod config;
pub mod keyboard;

use bindings::types::KeyBinding;
use bindings::utils::{create_binding_command, parse_binding_notification};
use bluetooth::characteristic::{get_write_characteristic, wait_for_notification};
use bluetooth::connect::connect_with_retry;
use btleplug::api::{Central, CharPropFlags, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::{Manager, Peripheral as BlePeripheral};
use config::{DEVICE_NAME, TIMEOUT};
use std::error::Error;

pub struct BleKeyboard {
    peripheral: BlePeripheral,
    characteristic: btleplug::api::Characteristic,
}

impl BleKeyboard {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let manager = Manager::new().await?;
        let adapter_list = manager.adapters().await?;

        if adapter_list.is_empty() {
            return Err("No Bluetooth adapters found".into());
        }

        let mut found_device = None;
        for adapter in adapter_list.iter() {
            adapter.start_scan(ScanFilter::default()).await?;
            let peripherals = adapter.peripherals().await?;

            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await?;
                if let Some(local_name) = properties.and_then(|p| p.local_name) {
                    if local_name == DEVICE_NAME {
                        found_device = Some(peripheral.clone());
                        break;
                    }
                }
            }
        }

        let peripheral = found_device.ok_or("Device not found")?;

        if !peripheral.is_connected().await? {
            connect_with_retry(&peripheral, 3).await?;
        }

        peripheral.discover_services().await?;
        let characteristic = get_write_characteristic(&peripheral).await?;

        Ok(BleKeyboard {
            peripheral,
            characteristic,
        })
    }

    pub async fn disconnect(&self) -> Result<(), Box<dyn Error>> {
        self.peripheral.disconnect().await?;
        Ok(())
    }

    pub async fn read_current_bindings(&self) -> Result<Vec<KeyBinding>, Box<dyn Error>> {
        if !self
            .characteristic
            .properties
            .contains(CharPropFlags::NOTIFY)
        {
            return Err("Characteristic does not support notifications".into());
        }

        self.peripheral.subscribe(&self.characteristic).await?;
        let mut notification_stream = self.peripheral.notifications().await?;
        let mut all_bindings = Vec::new();

        // Premier groupe de touches (1-8)
        self.peripheral
            .write(
                &self.characteristic,
                &bindings::commands::READ_BINDINGS_COMMANDS[0],
                btleplug::api::WriteType::WithResponse,
            )
            .await?;

        if let Ok(notification) = wait_for_notification(&mut notification_stream, TIMEOUT).await {
            let bindings = parse_binding_notification(&notification, 28, 60);
            all_bindings.extend(bindings);
        }

        // Deuxième groupe de touches (9-16)
        self.peripheral
            .write(
                &self.characteristic,
                &bindings::commands::READ_BINDINGS_COMMANDS[1],
                btleplug::api::WriteType::WithResponse,
            )
            .await?;

        if let Ok(notification) = wait_for_notification(&mut notification_stream, TIMEOUT).await {
            let bindings = parse_binding_notification(&notification, 23, 55);
            all_bindings.extend(bindings);
        }

        self.peripheral.unsubscribe(&self.characteristic).await?;
        Ok(all_bindings)
    }

    pub async fn write_bindings(&mut self, bindings: &[KeyBinding]) -> Result<(), Box<dyn Error>> {
        if bindings.len() != 16 {
            return Err("Expected exactly 16 key bindings".into());
        }

        // Première commande (touches 1-8)
        let cmd1 = create_binding_command(
            &bindings::commands::SET_BINDINGS_COMMANDS[0],
            &bindings[0..8],
            29,
            61,
        );
        self.peripheral
            .write(
                &self.characteristic,
                &cmd1,
                btleplug::api::WriteType::WithResponse,
            )
            .await?;

        // Deuxième commande (touches 9-16)
        let cmd2 = create_binding_command(
            &bindings::commands::SET_BINDINGS_COMMANDS[1],
            &bindings[8..16],
            24,
            56,
        );
        self.peripheral
            .write(
                &self.characteristic,
                &cmd2,
                btleplug::api::WriteType::WithResponse,
            )
            .await?;

        // Commandes finales
        for cmd in &bindings::commands::SET_BINDINGS_COMMANDS[2..] {
            self.peripheral
                .write(
                    &self.characteristic,
                    cmd,
                    btleplug::api::WriteType::WithResponse,
                )
                .await?;
        }

        Ok(())
    }
}
