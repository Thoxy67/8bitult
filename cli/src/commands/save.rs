use std::path::PathBuf;

use heigtbitult::config;

use super::{connect_device, disconnect_device, CommandResult};
use crate::{profile::Profile, ui};

pub async fn handle_save(
    name: String,
    description: Option<String>,
    output: Option<PathBuf>,
) -> CommandResult {
    let mut keyboard = connect_device().await?;

    ui::print_step("Reading current bindings...");
    let current_bindings = keyboard.read_current_bindings().await?;
    ui::print_bindings(&current_bindings, &config::BUTTON_NAMES);

    let profile = Profile::from_key_bindings(name.clone(), &current_bindings, description);
    let save_path = output.unwrap_or_else(|| Profile::get_default_save_path(&name));

    ui::print_step(&format!("Saving profile to {:?}...", save_path));
    profile.save_to_file(&save_path)?;
    ui::print_success("Profile saved successfully!");

    disconnect_device(&keyboard).await
}
