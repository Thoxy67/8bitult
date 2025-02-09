use crate::{profile::Profile, ui};
use anyhow::Result;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tar::{Archive, Builder};
use zstd::stream::{decode_all, encode_all};

pub fn import_profiles() -> Result<(), Box<dyn std::error::Error>> {
    ui::print_step("Importing profiles...");

    if let Some(config_dir) = dirs::config_dir() {
        let profile_dir = config_dir.join("8bitult").join("profiles");
        fs::create_dir_all(&profile_dir)?;

        // Lire tous les fichiers .tar.zst dans le répertoire courant
        for entry in fs::read_dir(".")? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("zst") {
                let file = File::open(&path)?;
                let decoded = decode_all(file)?;
                let mut archive = Archive::new(decoded.as_slice());
                archive.unpack(&profile_dir)?;

                ui::print_success(&format!("Imported profiles from {:?}", path));
            }
        }
    }

    Ok(())
}

pub fn export_profiles(save: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    ui::print_step("Exporting profiles...");

    let save_path = save.unwrap_or_else(|| PathBuf::from("backup.tar.zst"));
    let profiles = Profile::list_available_profiles()?;

    if profiles.is_empty() {
        ui::print_warning("No profiles found to export");
        return Ok(());
    }

    let mut file = File::create(&save_path)?;
    let mut builder = Builder::new(Vec::new());

    for (name, path) in profiles {
        let content = fs::read_to_string(&path)?;
        let filename = path.file_name().unwrap().to_str().unwrap();

        // Créer une entrée tar avec les données
        let mut header = tar::Header::new_gnu();
        header.set_size(content.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();

        builder.append_data(&mut header, filename, content.as_bytes())?;
        ui::print_success(&format!("Added profile: {}", name));
    }

    let tar_data = builder.into_inner()?;
    let compressed = encode_all(tar_data.as_slice(), 0)?;
    file.write_all(&compressed)?;

    ui::print_success(&format!("Profiles exported to {:?}", save_path));
    Ok(())
}
