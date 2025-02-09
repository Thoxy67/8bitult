use crate::config::CONNECT_TIMEOUT;
use btleplug::api::Peripheral;
use std::error::Error;
use tokio::time::Duration;
use tracing::{debug, error, info, warn};

pub async fn connect_with_retry(
    peripheral: &btleplug::platform::Peripheral,
    max_retries: u32,
) -> Result<(), Box<dyn Error>> {
    for retry in 0..max_retries {
        debug!("Attempting connection, try {}/{}", retry + 1, max_retries);
        match peripheral.connect().await {
            Ok(_) => {
                info!("Connected successfully!");
                tokio::time::sleep(CONNECT_TIMEOUT).await;
                return Ok(());
            }
            Err(e) => {
                if retry == max_retries - 1 {
                    error!("Connection failed after {} attempts: {}", max_retries, e);
                    return Err(e.into());
                }
                warn!("Connection attempt {} failed: {}", retry + 1, e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
    error!("Max connection retries exceeded");
    Err("Max connection retries exceeded".into())
}
