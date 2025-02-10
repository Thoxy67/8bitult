// read.rs
use std::error::Error;
use tokio::time::Duration;
use tracing::{debug, error, info, warn};

use btleplug::api::{CharPropFlags, Characteristic, Peripheral};

use crate::bindings::{
    commands::READ_BINDINGS_COMMANDS, types::KeyBinding, utils::parse_binding_notification,
};
use crate::bluetooth::characteristic::wait_for_notification;
use crate::config::{BUTTON_NAMES, TIMEOUT};
use crate::keyboard::KeyCode;

pub async fn read_current_bindings(
    peripheral: &btleplug::platform::Peripheral,
    characteristic: &Characteristic,
) -> Result<Vec<KeyBinding>, Box<dyn Error>> {
    if !characteristic.properties.contains(CharPropFlags::NOTIFY) {
        error!("Characteristic does not support notifications");
        return Err("Characteristic does not support notifications".into());
    }

    peripheral.subscribe(&characteristic).await?;
    let mut notification_stream = peripheral.notifications().await?;
    let mut all_bindings = Vec::new();

    // Premier groupe de touches (1-8)
    info!("Reading keys 1-8");
    for retry in 0..3 {
        debug!("Attempting to read first key group, attempt {}", retry + 1);
        match peripheral
            .write(
                &characteristic,
                &READ_BINDINGS_COMMANDS[0],
                btleplug::api::WriteType::WithResponse,
            )
            .await
        {
            Ok(_) => match wait_for_notification(&mut notification_stream, TIMEOUT).await {
                Ok(value) => {
                    let bindings = parse_binding_notification(&value, 28, 60);
                    // Log les bindings pour le debug
                    for (i, binding) in bindings.iter().enumerate() {
                        let key_names: Vec<String> = binding
                            .iter()
                            .map(|&key| {
                                KeyCode::from_u8(key)
                                    .map(|k| k.name())
                                    .unwrap_or_else(|| format!("Unknown({:#04x})", key))
                            })
                            .collect();
                        debug!(
                            button = BUTTON_NAMES[i],
                            key_number = i + 1,
                            keys = ?key_names,
                            "Key binding"
                        );
                    }
                    all_bindings.extend(bindings);
                    break;
                }
                Err(e) => {
                    if retry == 2 {
                        error!("Failed to receive notification after all retries: {}", e);
                        return Err(e.into());
                    }
                    warn!("Retry {} - Notification error: {}", retry + 1, e);
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            },
            Err(e) => {
                if retry == 2 {
                    error!("Failed to write command after all retries: {}", e);
                    return Err(e.into());
                }
                warn!("Retry {} - Write error: {}", retry + 1, e);
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }

    // Deuxième groupe de touches (9-16)
    info!("Reading keys 9-16");
    for retry in 0..3 {
        debug!("Attempting to read second key group, attempt {}", retry + 1);
        match peripheral
            .write(
                &characteristic,
                &READ_BINDINGS_COMMANDS[1],
                btleplug::api::WriteType::WithResponse,
            )
            .await
        {
            Ok(_) => match wait_for_notification(&mut notification_stream, TIMEOUT).await {
                Ok(value) => {
                    let bindings = parse_binding_notification(&value, 23, 55);
                    // Log les bindings pour le debug
                    for (i, binding) in bindings.iter().enumerate() {
                        let key_names: Vec<String> = binding
                            .iter()
                            .map(|&key| {
                                KeyCode::from_u8(key)
                                    .map(|k| k.name())
                                    .unwrap_or_else(|| format!("Unknown({:#04x})", key))
                            })
                            .collect();
                        debug!(
                            button = BUTTON_NAMES[i + 8],
                            key_number = i + 9,
                            keys = ?key_names,
                            "Key binding"
                        );
                    }
                    all_bindings.extend(bindings);
                    break;
                }
                Err(e) => {
                    if retry == 2 {
                        error!("Failed to receive notification after all retries: {}", e);
                        return Err(e.into());
                    }
                    warn!("Retry {} - Notification error: {}", retry + 1, e);
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            },
            Err(e) => {
                if retry == 2 {
                    error!("Failed to write command after all retries: {}", e);
                    return Err(e.into());
                }
                warn!("Retry {} - Write error: {}", retry + 1, e);
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }

    // Envoyer la commande de reset sans unsubscribe
    crate::bindings::reset::send_reset_command(peripheral, characteristic, true).await?;

    info!("Successfully read all bindings");
    Ok(all_bindings)
}
