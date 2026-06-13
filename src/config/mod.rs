//! Compile-time configuration: the device name, the timeouts, and the button
//! order used everywhere bindings are read or written.

use tokio::time::Duration;

/// Bluetooth name the 8BitDo Micro advertises under while in keyboard mode.
pub const DEVICE_NAME: &str = "80EL";
/// How long to wait for a notification after sending a read command.
pub const TIMEOUT: Duration = Duration::from_millis(1000);
/// Settle time after a successful connection before talking to the device.
pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(2);

/// Maximum time spent scanning for the target device before giving up.
pub const SCAN_TIMEOUT: Duration = Duration::from_secs(15);

/// The 16 buttons in the order bindings are stored and reported. Index 0 is
/// `A`, index 15 is `RIGHT`. Profiles and the device protocol both use this
/// order.
pub const BUTTON_NAMES: [&str; 16] = [
    "A", "B", "X", "Y", "L1", "R1", "L2", "R2", "SELECT", "START", "HOME", "LOGO", "UP", "DOWN",
    "LEFT", "RIGHT",
];
