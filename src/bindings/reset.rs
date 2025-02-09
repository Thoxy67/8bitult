use btleplug::api::{Characteristic, Peripheral, WriteType};
use std::error::Error;
use tracing::{debug, info};

use crate::bindings::commands::RESET_COMMAND;

pub async fn send_reset_command(
    peripheral: &btleplug::platform::Peripheral,
    characteristic: &Characteristic,
    should_unsubscribe: bool,
) -> Result<(), Box<dyn Error>> {
    info!("Sending reset commands");
    for (i, cmd) in RESET_COMMAND.iter().enumerate() {
        debug!("Sending reset command {}/{}", i + 1, RESET_COMMAND.len());
        peripheral
            .write(characteristic, cmd, WriteType::WithResponse)
            .await?;
    }

    if should_unsubscribe {
        debug!("Unsubscribing from characteristic");
        peripheral.unsubscribe(characteristic).await?;
    }

    info!("Reset completed successfully");
    Ok(())
}
