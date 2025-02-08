use crate::config::CONNECT_TIMEOUT;
use btleplug::api::Peripheral;
use std::error::Error;
use tokio::time::Duration;

pub async fn connect_with_retry(
    peripheral: &btleplug::platform::Peripheral,
    max_retries: u32,
) -> Result<(), Box<dyn Error>> {
    for retry in 0..max_retries {
        match peripheral.connect().await {
            Ok(_) => {
                println!("Connected successfully!");
                // Augmenter le temps d'attente après la connexion
                tokio::time::sleep(CONNECT_TIMEOUT).await;
                return Ok(());
            }
            Err(e) => {
                if retry == max_retries - 1 {
                    return Err(e.into());
                }
                println!("Connection attempt {} failed: {}", retry + 1, e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
    Err("Max connection retries exceeded".into())
}
