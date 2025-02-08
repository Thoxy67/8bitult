use btleplug::api::{CharPropFlags, Characteristic, Peripheral};
use futures::StreamExt;
use std::error::Error;
use tokio::time::timeout;

pub async fn get_write_characteristic(
    peripheral: &btleplug::platform::Peripheral,
) -> Result<Characteristic, Box<dyn Error>> {
    for service in peripheral.services() {
        for characteristic in service.characteristics {
            if characteristic.properties.contains(CharPropFlags::WRITE) {
                return Ok(characteristic);
            }
        }
    }
    Err("No writable characteristic found".into())
}

pub async fn wait_for_notification(
    notification_stream: &mut (impl StreamExt<Item = btleplug::api::ValueNotification> + Unpin),
    timeout_duration: tokio::time::Duration,
) -> Result<Vec<u8>, Box<dyn Error>> {
    match timeout(timeout_duration, notification_stream.next()).await {
        Ok(Some(notification)) => Ok(notification.value),
        Ok(None) => Err("No notification received".into()),
        Err(_) => Err("Notification timeout".into()),
    }
}
