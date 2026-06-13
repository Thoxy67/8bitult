use colored::Colorize;
use heigtbitult::config;

use super::{connect_device, disconnect_device, CommandResult};
use crate::{profile::Profile, ui};

pub async fn handle_read() -> CommandResult {
    let mut keyboard = connect_device().await?;

    ui::print_step("Reading current bindings...");
    let current_bindings = keyboard.read_current_bindings().await?;

    // Tell the user if what is on the device matches one of their profiles.
    let profiles = Profile::list_available_profiles()?;
    for (profile, _path) in profiles {
        if let Ok(profile_bindings) = profile.to_key_bindings() {
            if current_bindings == profile_bindings {
                ui::print_success(&format!(
                    "Current configuration matches profile: {}",
                    profile.name.bold()
                ));
                break;
            }
        }
    }

    ui::print_bindings(&current_bindings, &config::BUTTON_NAMES);

    disconnect_device(&keyboard).await
}
