use colored::Colorize;

use super::CommandResult;
use crate::profile::Profile;

pub async fn handle_list() -> CommandResult {
    let profiles = Profile::list_available_profiles()?;
    if profiles.is_empty() {
        println!("\nNo profiles found.");
        return Ok(());
    }

    println!("\nAvailable profiles:");
    for (profile, path) in profiles {
        match profile.description {
            Some(desc) => println!(
                "  - {} ({})\n    {}",
                profile.name.bold(),
                path.display(),
                desc.bright_black()
            ),
            None => println!("  - {} ({})", profile.name.bold(), path.display()),
        }
    }
    Ok(())
}
