use anyhow::Result;
use backup::{export_profiles, import_profiles};
use colored::Colorize;
use core::time;
use heigtbitult::{config, BleKeyboard};
use std::path::PathBuf;
use tokio::time::sleep;

mod backup;

use crate::{profile::Profile, ui};

pub async fn handle_backup(
    import: bool,
    export: bool,
    save: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if import {
        import_profiles()?;
    } else if export {
        export_profiles(save)?;
    }
    Ok(())
}

pub async fn handle_list() -> Result<(), Box<dyn std::error::Error>> {
    let profiles = Profile::list_available_profiles()?;
    if profiles.is_empty() {
        println!("\nNo profiles found.");
    } else {
        println!("\nAvailable profiles:");
        for (name, path) in profiles {
            if let Ok(profile) = Profile::load(path.to_str().unwrap()) {
                match profile.description {
                    Some(desc) => println!(
                        "  - {} ({})\n    {}",
                        name.bold(),
                        path.display(),
                        desc.bright_black()
                    ),
                    None => println!("  - {} ({})", name.bold(), path.display()),
                }
            }
        }
    }
    Ok(())
}

pub async fn handle_read() -> Result<(), Box<dyn std::error::Error>> {
    ui::print_step("Searching for device...");
    let mut keyboard = BleKeyboard::new().await?;
    ui::print_success("Device connected successfully!");

    ui::print_step("Reading current bindings...");
    let current_bindings = keyboard.read_current_bindings().await?;

    // Vérifier si la configuration correspond à un profil existant
    let profiles = Profile::list_available_profiles()?;
    for (name, path) in profiles {
        if let Ok(profile) = Profile::load(path.to_str().unwrap()) {
            if let Ok(profile_bindings) = profile.to_key_bindings() {
                if current_bindings == profile_bindings {
                    ui::print_success(&format!(
                        "Current configuration matches profile: {}",
                        name.bold()
                    ));
                    break;
                }
            }
        }
    }

    println!("\nCurrent bindings configuration:");
    ui::print_bindings(&current_bindings, &config::BUTTON_NAMES);

    ui::print_step("Disconnecting device...");
    keyboard.disconnect().await?;
    ui::print_success("Device disconnected successfully");
    Ok(())
}

pub async fn handle_attach(
    profile_name: Option<String>,
    config_file: Option<std::path::PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    ui::print_step("Searching for device...");
    let mut keyboard = BleKeyboard::new().await?;
    ui::print_success("Device connected successfully!");

    if let Some(profile) =
        Profile::load_from_name_or_path(profile_name.as_deref(), config_file.as_ref())?
    {
        ui::print_success(&format!("Loaded profile: '{}'", profile.name));

        let new_bindings = profile.to_key_bindings()?;

        ui::print_step("Writing new bindings...");
        keyboard.write_bindings(&new_bindings).await?;
        ui::print_success("Bindings written successfully");

        sleep(time::Duration::from_secs(2)).await;

        ui::print_step("Verifying new bindings...");
        let updated_bindings = keyboard.read_current_bindings().await?;

        let matches = new_bindings
            .iter()
            .zip(updated_bindings.iter())
            .all(|(a, b)| a == b);

        println!("\nNew bindings configuration:");
        ui::print_bindings(&updated_bindings, &config::BUTTON_NAMES);

        if !matches {
            ui::print_warning("Warning: Some bindings might not have been applied correctly");
        } else {
            ui::print_success("All bindings verified successfully");
        }
    } else {
        ui::print_warning("No profile specified, skipping write");
    }

    ui::print_step("Disconnecting device...");
    keyboard.disconnect().await?;
    ui::print_success("Device disconnected successfully");
    Ok(())
}

pub async fn handle_save(
    name: String,
    description: Option<String>,
    output: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    ui::print_step("Searching for device...");
    let mut keyboard = BleKeyboard::new().await?;
    ui::print_success("Device connected successfully!");

    ui::print_step("Reading current bindings...");
    let current_bindings = keyboard.read_current_bindings().await?;

    println!("\nCurrent bindings configuration:");
    ui::print_bindings(&current_bindings, &config::BUTTON_NAMES);

    let profile = Profile::from_key_bindings(name.clone(), &current_bindings, description);

    let save_path = output.unwrap_or_else(|| Profile::get_default_save_path(&name));

    ui::print_step(&format!("Saving profile to {:?}...", save_path));
    profile.save_to_file(&save_path)?;
    ui::print_success("Profile saved successfully!");

    ui::print_step("Disconnecting device...");
    keyboard.disconnect().await?;
    ui::print_success("Device disconnected successfully");
    Ok(())
}
