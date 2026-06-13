use std::path::PathBuf;

use heigtbitult::config;
use tokio::time::{sleep, Duration};

use super::{connect_device, disconnect_device, CommandResult};
use crate::{profile::Profile, ui};

pub async fn handle_attach(
    profile_name: Option<String>,
    config_file: Option<PathBuf>,
) -> CommandResult {
    let mut keyboard = connect_device().await?;

    match Profile::load_from_name_or_path(profile_name.as_deref(), config_file.as_ref())? {
        Some(profile) => {
            ui::print_success(&format!("Loaded profile: '{}'", profile.name));

            let new_bindings = profile.to_key_bindings()?;

            ui::print_step("Writing new bindings...");
            keyboard.write_bindings(&new_bindings).await?;
            ui::print_success("Bindings written successfully");

            // Give the device a moment to apply the change before reading back.
            sleep(Duration::from_secs(2)).await;

            ui::print_step("Verifying new bindings...");
            let updated_bindings = keyboard.read_current_bindings().await?;

            println!("\nNew bindings configuration:");
            ui::print_bindings(&updated_bindings, &config::BUTTON_NAMES);

            if new_bindings == updated_bindings {
                ui::print_success("All bindings verified successfully");
            } else {
                ui::print_warning("Warning: Some bindings might not have been applied correctly");
            }
        }
        None => ui::print_warning("No profile specified, skipping write"),
    }

    disconnect_device(&keyboard).await
}
