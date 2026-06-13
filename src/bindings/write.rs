// write.rs
use std::error::Error;
use tokio::time::Duration;
use tracing::{debug, error, info, warn};

use btleplug::api::{Characteristic, Peripheral, WriteType};

use crate::bindings::{
    commands::SET_BINDINGS_COMMANDS, types::KeyBinding, utils::create_binding_command,
};

/// Writes a single command to the device, retrying up to 3 times.
/// `label` is used only for log messages.
async fn write_with_retry(
    peripheral: &btleplug::platform::Peripheral,
    characteristic: &Characteristic,
    command: &[u8],
    label: &str,
) -> Result<(), Box<dyn Error>> {
    for retry in 0..3 {
        debug!("Sending {}, attempt {}", label, retry + 1);
        match peripheral
            .write(characteristic, command, WriteType::WithResponse)
            .await
        {
            Ok(_) => {
                debug!("{} sent successfully", label);
                return Ok(());
            }
            Err(e) => {
                if retry == 2 {
                    error!("Failed to send {} after all retries: {}", label, e);
                    return Err(e.into());
                }
                warn!("Retry {} - Write error: {}", retry + 1, e);
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }
    Err(format!("Failed to send {} after all retries", label).into())
}

pub async fn write_new_bindings(
    peripheral: &btleplug::platform::Peripheral,
    characteristic: &Characteristic,
    bindings: &[KeyBinding],
) -> Result<(), Box<dyn Error>> {
    info!("Starting bindings modification");

    // First command (keys 1-8).
    let cmd1 = create_binding_command(SET_BINDINGS_COMMANDS[0], &bindings[0..8], 29, 61);
    write_with_retry(peripheral, characteristic, &cmd1, "command 1 (keys 1-8)").await?;

    // Second command (keys 9-16).
    let cmd2 = create_binding_command(SET_BINDINGS_COMMANDS[1], &bindings[8..16], 24, 56);
    write_with_retry(peripheral, characteristic, &cmd2, "command 2 (keys 9-16)").await?;

    // Final commands.
    info!("Sending final commands");
    for (i, cmd) in SET_BINDINGS_COMMANDS[2..].iter().enumerate() {
        write_with_retry(
            peripheral,
            characteristic,
            cmd,
            &format!("final command {}", i + 1),
        )
        .await?;
    }

    // Send the reset command and unsubscribe.
    crate::bindings::reset::send_reset_command(peripheral, characteristic, false).await?;

    info!("All binding modifications completed successfully");
    Ok(())
}
