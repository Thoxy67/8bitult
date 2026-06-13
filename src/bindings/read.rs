// read.rs
use std::error::Error;
use tokio::time::Duration;
use tracing::{debug, error, info, warn};

use btleplug::api::{CharPropFlags, Characteristic, Peripheral, ValueNotification, WriteType};
use futures::Stream;

use crate::bindings::{
    commands::READ_BINDINGS_COMMANDS, types::KeyBinding, utils::parse_binding_notification,
};
use crate::bluetooth::characteristic::wait_for_notification;
use crate::config::{BUTTON_NAMES, TIMEOUT};
use crate::keyboard::KeyCode;

/// Reads a single key group: writes `command`, waits for the device
/// notification and parses the bindings in `[start_pos, end_pos)`.
///
/// `button_offset` is the index of the first button in this group, used only
/// for debug logging. The whole exchange is retried up to 3 times.
async fn read_key_group(
    peripheral: &btleplug::platform::Peripheral,
    characteristic: &Characteristic,
    notification_stream: &mut (impl Stream<Item = ValueNotification> + Unpin),
    command: &[u8],
    start_pos: usize,
    end_pos: usize,
    button_offset: usize,
) -> Result<Vec<KeyBinding>, Box<dyn Error>> {
    for retry in 0..3 {
        debug!(attempt = retry + 1, button_offset, "Reading key group");

        if let Err(e) = peripheral
            .write(characteristic, command, WriteType::WithResponse)
            .await
        {
            if retry == 2 {
                error!("Failed to write command after all retries: {}", e);
                return Err(e.into());
            }
            warn!("Retry {} - Write error: {}", retry + 1, e);
            tokio::time::sleep(Duration::from_millis(500)).await;
            continue;
        }

        match wait_for_notification(notification_stream, TIMEOUT).await {
            Ok(value) => {
                let bindings = parse_binding_notification(&value, start_pos, end_pos);
                for (i, binding) in bindings.iter().enumerate() {
                    let key_names: Vec<String> =
                        binding.iter().map(|&key| KeyCode::display_name(key)).collect();
                    debug!(
                        button = BUTTON_NAMES[button_offset + i],
                        key_number = button_offset + i + 1,
                        keys = ?key_names,
                        "Key binding"
                    );
                }
                return Ok(bindings);
            }
            Err(e) => {
                if retry == 2 {
                    error!("Failed to receive notification after all retries: {}", e);
                    return Err(e);
                }
                warn!("Retry {} - Notification error: {}", retry + 1, e);
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }

    Err("Failed to read key group after all retries".into())
}

pub async fn read_current_bindings(
    peripheral: &btleplug::platform::Peripheral,
    characteristic: &Characteristic,
) -> Result<Vec<KeyBinding>, Box<dyn Error>> {
    if !characteristic.properties.contains(CharPropFlags::NOTIFY) {
        error!("Characteristic does not support notifications");
        return Err("Characteristic does not support notifications".into());
    }

    peripheral.subscribe(characteristic).await?;
    let mut notification_stream = peripheral.notifications().await?;
    let mut all_bindings = Vec::with_capacity(16);

    // First button group (1-8).
    info!("Reading keys 1-8");
    all_bindings.extend(
        read_key_group(
            peripheral,
            characteristic,
            &mut notification_stream,
            READ_BINDINGS_COMMANDS[0],
            28,
            60,
            0,
        )
        .await?,
    );

    // Second button group (9-16).
    info!("Reading keys 9-16");
    all_bindings.extend(
        read_key_group(
            peripheral,
            characteristic,
            &mut notification_stream,
            READ_BINDINGS_COMMANDS[1],
            23,
            55,
            8,
        )
        .await?,
    );

    // Send the reset command but stay subscribed.
    crate::bindings::reset::send_reset_command(peripheral, characteristic, true).await?;

    info!("Successfully read all bindings");
    Ok(all_bindings)
}
