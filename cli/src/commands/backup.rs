use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use tar::{Archive, Builder};
use zstd::{decode_all, encode_all};

use super::CommandResult;
use crate::{profile::Profile, ui};

pub async fn handle_backup(import: Option<PathBuf>, export: Option<PathBuf>) -> CommandResult {
    match (import, export) {
        (Some(path), None) => import_profiles(path),
        (None, Some(path)) => export_profiles(path),
        _ => {
            println!("Please specify either --import <path> or --export <path>");
            println!("\nExample usage:");
            println!("  Import: 8bitult-cli backup --import backup.tar.zst");
            println!("  Export: 8bitult-cli backup --export backup.tar.zst");
            Ok(())
        }
    }
}

/// Unpacks a profile archive into the user's profiles directory.
fn import_profiles(path: PathBuf) -> CommandResult {
    ui::print_step(&format!("Importing profiles from {:?}...", path));

    let Some(config_dir) = dirs::config_dir() else {
        ui::print_warning("Could not determine config directory, nothing imported");
        return Ok(());
    };

    let decoded = decode_all(File::open(path)?)?;
    let profile_dir = config_dir.join("8bitult").join("profiles");
    fs::create_dir_all(&profile_dir)?;
    Archive::new(decoded.as_slice()).unpack(&profile_dir)?;

    ui::print_success("Profiles imported successfully");
    Ok(())
}

/// Packs every known profile into a compressed tar archive.
fn export_profiles(save_path: PathBuf) -> CommandResult {
    ui::print_step("Exporting profiles...");

    let profiles = Profile::list_available_profiles()?;
    if profiles.is_empty() {
        ui::print_warning("No profiles found to export");
        return Ok(());
    }

    let mut builder = Builder::new(Vec::new());
    for (profile, path) in profiles {
        let content = fs::read_to_string(&path)?;
        let filename = path.file_name().unwrap().to_str().unwrap();

        let mut header = tar::Header::new_gnu();
        header.set_size(content.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();

        builder.append_data(&mut header, filename, content.as_bytes())?;
        ui::print_success(&format!("Added profile: {}", profile.name));
    }

    let compressed = encode_all(builder.into_inner()?.as_slice(), 0)?;
    File::create(&save_path)?.write_all(&compressed)?;

    ui::print_success(&format!("Profiles exported to {:?}", save_path));
    Ok(())
}
