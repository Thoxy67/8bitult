use crate::{profile::Profile, ui};
use anyhow::Result;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tar::Builder;
use zstd::stream::encode_all;

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
