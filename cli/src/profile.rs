use anyhow::{Context, Result};
use colored::Colorize;
use heigtbitult::config;
use heigtbitult::{bindings::types::KeyBinding, keyboard::*};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

pub fn key_name_to_value(key_name: &str) -> anyhow::Result<u8> {
    // Raw hex codes, written as keycode(XX).
    if let Some(hex_str) = key_name
        .strip_prefix("keycode(")
        .and_then(|s| s.strip_suffix(")"))
    {
        let value = u8::from_str_radix(hex_str, 16)
            .with_context(|| format!("Invalid hexadecimal format: {}. Expected format: keycode(XX) where XX is a hexadecimal number", hex_str))?;

        return Ok(value);
    }

    // Otherwise match the name against the keycode table.
    for i in 0..=u8::MAX {
        if let Some(key) = KeyCode::from_u8(i) {
            if key.name() == key_name {
                return Ok(i);
            }
        }
    }

    Err(anyhow::anyhow!("Invalid key name: {}", key_name))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub name: String,
    pub description: Option<String>,
    pub bindings: HashMap<String, Vec<String>>,
}

impl Profile {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read profile from {}", path.display()))?;
        toml::from_str(&content)
            .with_context(|| format!("Failed to parse TOML from {}", path.display()))
    }

    /// Loads every readable profile from the search directories, returning each
    /// parsed profile alongside the file it came from.
    pub fn list_available_profiles() -> Result<Vec<(Profile, PathBuf)>> {
        let mut profiles = Vec::new();

        let search_paths = [
            PathBuf::from("./profiles"),
            dirs::config_dir()
                .map(|mut p| {
                    p.push("8bitult");
                    p.push("profiles");
                    p
                })
                .unwrap_or_default(),
        ];

        for base_path in search_paths {
            if !base_path.exists() {
                continue;
            }
            for entry in fs::read_dir(&base_path)? {
                let path = entry?.path();
                if path.is_file() && path.extension() == Some(OsStr::new("toml")) {
                    if let Ok(profile) = Self::load(&path) {
                        profiles.push((profile, path));
                    }
                }
            }
        }

        Ok(profiles)
    }

    fn find_profile(name: &str) -> Result<Profile> {
        let mut profiles = Self::list_available_profiles()?;

        // Match against the `name` field inside each TOML.
        if let Some(pos) = profiles.iter().position(|(profile, _)| profile.name == name) {
            Ok(profiles.swap_remove(pos).0)
        } else {
            println!("\nAvailable profiles:");
            for (profile, path) in profiles {
                println!("  - {} ({})", profile.name.bold(), path.display());
            }
            Err(anyhow::anyhow!("Profile '{}' not found", name))
        }
    }

    pub fn load_from_name_or_path(
        profile_name: Option<&str>,
        config_path: Option<&PathBuf>,
    ) -> Result<Option<Self>> {
        match (profile_name, config_path) {
            // Look the profile up by name in the search directories.
            (Some(name), None) => Ok(Some(Self::find_profile(name)?)),
            (None, Some(path)) => Ok(Some(Self::load(path)?)),
            (None, None) => Ok(None),
            (Some(_), Some(_)) => Err(anyhow::anyhow!(
                "Cannot specify both profile name and config file"
            )),
        }
    }

    pub fn to_key_bindings(&self) -> Result<Vec<KeyBinding>> {
        let button_map: HashMap<_, _> = config::BUTTON_NAMES
            .iter()
            .enumerate()
            .map(|(i, &name)| (name, i))
            .collect();

        let mut result = vec![[KeyCode::None.to_u8(); 4]; 16];

        for (button, keys) in &self.bindings {
            let idx = *button_map
                .get(button.as_str())
                .with_context(|| format!("Invalid button name: {}", button))?;

            for (i, key_name) in keys.iter().enumerate().take(4) {
                result[idx][i] = key_name_to_value(key_name.as_str()).with_context(|| {
                    format!("Invalid key name for button {}: {}", button, key_name)
                })?;
            }
        }

        Ok(result)
    }

    pub fn from_key_bindings(
        name: String,
        bindings: &[KeyBinding],
        description: Option<String>,
    ) -> Self {
        let mut binding_map = HashMap::new();

        for (i, binding) in bindings.iter().enumerate() {
            if binding.iter().any(|&k| k != KeyCode::None.to_u8()) {
                let button_name = config::BUTTON_NAMES[i].to_string();
                let key_names: Vec<String> =
                    binding.iter().map(|&key| KeyCode::display_name(key)).collect();
                binding_map.insert(button_name, key_names);
            }
        }

        Self {
            name,
            description,
            bindings: binding_map,
        }
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        // Create the parent directory if it does not exist yet.
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }

        let toml = toml::to_string_pretty(&self)?;
        fs::write(path, toml)?;
        Ok(())
    }

    pub fn get_default_save_path(name: &str) -> PathBuf {
        let file_name = format!("{}.toml", name.to_lowercase().replace(' ', "_"));
        let mut dir = dirs::config_dir()
            .map(|mut p| {
                p.push("8bitult");
                p.push("profiles");
                p
            })
            .unwrap_or_else(|| PathBuf::from("profiles"));
        dir.push(file_name);
        dir
    }
}
