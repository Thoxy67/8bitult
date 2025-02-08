use std::error::Error;
use tokio::time::Duration;

use btleplug::api::{CharPropFlags, Characteristic, Peripheral};

use crate::bindings::commands::READ_BINDINGS_COMMANDS;
use crate::bindings::utils::parse_binding_notification;
use crate::bluetooth::characteristic::wait_for_notification;
use crate::config::{BUTTON_NAMES, TIMEOUT};
use crate::keyboard;

pub async fn read_current_bindings(
    peripheral: &btleplug::platform::Peripheral,
    characteristic: &Characteristic,
) -> Result<(), Box<dyn Error>> {
    if characteristic.properties.contains(CharPropFlags::NOTIFY) {
        peripheral.subscribe(&characteristic).await?;
        let mut notification_stream = peripheral.notifications().await?;

        // Premier groupe de touches (1-8)
        println!("\n--- Lecture des touches 1-8 ---");
        for retry in 0..3 {
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
                        for (i, binding) in bindings.iter().enumerate() {
                            let key_names: Vec<String> = binding
                                .iter()
                                .map(|&key| keyboard::get_key_name(key))
                                .collect();
                            println!(
                                "  {} (Touche {}): [{}]",
                                BUTTON_NAMES[i],
                                i + 1,
                                key_names.join(", ")
                            );
                        }
                        break;
                    }
                    Err(e) => {
                        if retry == 2 {
                            return Err(e);
                        }
                        println!("Retry {} - Notification error: {}", retry + 1, e);
                        tokio::time::sleep(Duration::from_millis(500)).await;
                    }
                },
                Err(e) => {
                    if retry == 2 {
                        return Err(e.into());
                    }
                    println!("Retry {} - Write error: {}", retry + 1, e);
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            }
        }

        // Deuxième groupe de touches (9-16)
        println!("\n--- Lecture des touches 9-16 ---");
        for retry in 0..3 {
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
                        for (i, binding) in bindings.iter().enumerate() {
                            let key_names: Vec<String> = binding
                                .iter()
                                .map(|&key| keyboard::get_key_name(key))
                                .collect();
                            println!("  {} : [{}]", BUTTON_NAMES[i + 8], key_names.join(", "));
                        }
                        break;
                    }
                    Err(e) => {
                        if retry == 2 {
                            return Err(e);
                        }
                        println!("Retry {} - Notification error: {}", retry + 1, e);
                        tokio::time::sleep(Duration::from_millis(500)).await;
                    }
                },
                Err(e) => {
                    if retry == 2 {
                        return Err(e.into());
                    }
                    println!("Retry {} - Write error: {}", retry + 1, e);
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            }
        }

        peripheral.unsubscribe(&characteristic).await?;
    }
    Ok(())
}
