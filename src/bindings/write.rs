// write.rs
use std::error::Error;
use tokio::time::Duration;
use tracing::{debug, error, info, warn};

use btleplug::api::{Characteristic, Peripheral};

use crate::bindings::{
    commands::SET_BINDINGS_COMMANDS, types::KeyBinding, utils::create_binding_command,
};

pub async fn write_new_bindings(
    peripheral: &btleplug::platform::Peripheral,
    characteristic: &Characteristic,
    bindings: &[KeyBinding],
) -> Result<(), Box<dyn Error>> {
    info!("Starting bindings modification");

    // Première commande (touches 1-8)
    let cmd1 = create_binding_command(&SET_BINDINGS_COMMANDS[0], &bindings[0..8], 29, 61);
    for retry in 0..3 {
        debug!(
            "Sending command 1 for modification (keys 1-8), attempt {}",
            retry + 1
        );
        match peripheral
            .write(
                &characteristic,
                &cmd1,
                btleplug::api::WriteType::WithResponse,
            )
            .await
        {
            Ok(_) => {
                debug!("Command 1 sent successfully");
                break;
            }
            Err(e) => {
                if retry == 2 {
                    error!("Failed to send command 1 after all retries: {}", e);
                    return Err(e.into());
                }
                warn!("Retry {} - Write error: {}", retry + 1, e);
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }

    // Deuxième commande (touches 9-16)
    let cmd2 = create_binding_command(&SET_BINDINGS_COMMANDS[1], &bindings[8..16], 24, 56);
    for retry in 0..3 {
        debug!(
            "Sending command 2 for modification (keys 9-16), attempt {}",
            retry + 1
        );
        match peripheral
            .write(
                &characteristic,
                &cmd2,
                btleplug::api::WriteType::WithResponse,
            )
            .await
        {
            Ok(_) => {
                debug!("Command 2 sent successfully");
                break;
            }
            Err(e) => {
                if retry == 2 {
                    error!("Failed to send command 2 after all retries: {}", e);
                    return Err(e.into());
                }
                warn!("Retry {} - Write error: {}", retry + 1, e);
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }

    // Commandes finales
    info!("Sending final commands");
    for (i, cmd) in SET_BINDINGS_COMMANDS[2..].iter().enumerate() {
        for retry in 0..3 {
            debug!("Sending final command {}, attempt {}", i + 1, retry + 1);
            match peripheral
                .write(&characteristic, cmd, btleplug::api::WriteType::WithResponse)
                .await
            {
                Ok(_) => {
                    debug!("Final command {} sent successfully", i + 1);
                    break;
                }
                Err(e) => {
                    if retry == 2 {
                        error!(
                            "Failed to send final command {} after all retries: {}",
                            i + 1,
                            e
                        );
                        return Err(e.into());
                    }
                    warn!("Retry {} - Write error: {}", retry + 1, e);
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            }
        }
    }

    // Envoyer la commande de reset et unsubscribe
    crate::bindings::reset::send_reset_command(peripheral, characteristic, false).await?;

    info!("All binding modifications completed successfully");
    Ok(())
}
