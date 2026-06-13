//! Command handlers, one module per subcommand.

use crate::ui;
use heigtbitult::BleKeyboard;

mod attach;
mod backup;
mod list;
mod read;
mod save;

pub use attach::handle_attach;
pub use backup::handle_backup;
pub use list::handle_list;
pub use read::handle_read;
pub use save::handle_save;

/// Return type shared by every command handler.
pub type CommandResult = Result<(), Box<dyn std::error::Error>>;

/// Connects to the controller, printing the usual progress lines.
async fn connect_device() -> Result<BleKeyboard, Box<dyn std::error::Error>> {
    ui::print_step("Searching for device...");
    let keyboard = BleKeyboard::new().await?;
    ui::print_success("Device connected successfully!");
    Ok(keyboard)
}

/// Disconnects from the controller, printing the usual progress lines.
async fn disconnect_device(keyboard: &BleKeyboard) -> CommandResult {
    ui::print_step("Disconnecting device...");
    keyboard.disconnect().await?;
    ui::print_success("Device disconnected successfully");
    Ok(())
}
