use tokio::time::Duration;

pub const DEVICE_NAME: &str = "80EL";
pub const TIMEOUT: Duration = Duration::from_millis(1000);
pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(2);

pub const BUTTON_NAMES: [&str; 16] = [
    "A", "B", "X", "Y", "L1", "R1", "L2", "R2", "SELECT", "START", "HOME", "LOGO", "UP", "DOWN",
    "LEFT", "RIGHT",
];
