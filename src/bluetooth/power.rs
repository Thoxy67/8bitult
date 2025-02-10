use std::error::Error;
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, info};

#[cfg(target_os = "linux")]
pub async fn ensure_bluetooth_enabled() -> Result<(), Box<dyn Error>> {
    // Vérifie l'état via le cache de bluetoothctl (commande rapide)

    use std::process::Stdio;
    let status = Command::new("bluetoothctl")
        .arg("show")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()?;
    let status_str = String::from_utf8_lossy(&status.stdout);

    // Si déjà activé et en scan, on ne fait rien
    if status_str.contains("Powered: yes") {
        if status_str.contains("Discovering: yes") {
            debug!("Bluetooth already powered and scanning");
            return Ok(());
        }
        // Juste besoin de scanner
        debug!("Bluetooth powered, starting scan");
        Command::new("bluetoothctl")
            .args(["scan", "on"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        return Ok(());
    }

    // Si on arrive ici, il faut tout initialiser
    info!("Bluetooth needs initialization");
    Command::new("bluetoothctl")
        .args(["power", "on"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    sleep(Duration::from_secs(1)).await;

    Command::new("bluetoothctl")
        .args(["scan", "on"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    sleep(Duration::from_millis(500)).await;

    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub async fn ensure_bluetooth_enabled() -> Result<(), Box<dyn Error>> {
    Ok(())
}
