use anyhow::{Context, Result};
use colored::Colorize;
use heigtbitult::config;
use heigtbitult::{bindings::types::KeyBinding, keyboard::*};
use serde::Serialize;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

pub fn key_name_to_value(key_name: &str) -> anyhow::Result<u8> {
    // Gestion des codes hexadécimaux directs
    if let Some(hex_str) = key_name
        .strip_prefix("keycode(")
        .and_then(|s| s.strip_suffix(")"))
    {
        let value = u8::from_str_radix(hex_str, 16)
            .with_context(|| format!("Invalid hexadecimal format: {}. Expected format: keycode(XX) where XX is a hexadecimal number", hex_str))?;

        if value > u8::MAX {
            return Err(anyhow::anyhow!(
                "Keycode value {} is too large for u8 (max: {})",
                value,
                u8::MAX
            ));
        }

        return Ok(value as u8);
    }

    // Essayer de convertir à partir du nom
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
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read profile from {}", path))?;
        toml::from_str(&content).with_context(|| format!("Failed to parse TOML from {}", path))
    }

    pub fn list_available_profiles() -> Result<Vec<(String, PathBuf)>> {
        let mut profiles = Vec::new();

        let search_paths = vec![
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
            if base_path.exists() {
                for entry in fs::read_dir(&base_path)? {
                    let entry = entry?;
                    let path = entry.path();

                    // Vérifier si c'est un fichier .toml
                    if path.is_file() && path.extension() == Some(OsStr::new("toml")) {
                        // Charger le profil pour obtenir son nom
                        if let Ok(content) = fs::read_to_string(&path) {
                            if let Ok(profile) = toml::from_str::<Profile>(&content) {
                                profiles.push((profile.name, path));
                            }
                        }
                    }
                }
            }
        }

        Ok(profiles)
    }

    fn find_profile(name: &str) -> Result<PathBuf> {
        let profiles = Self::list_available_profiles()?;

        // Chercher le profil par son nom dans le TOML
        if let Some((_, path)) = profiles
            .iter()
            .find(|(profile_name, _)| profile_name == name)
        {
            Ok(path.clone())
        } else {
            println!("\nAvailable profiles:");
            for (name, path) in profiles {
                println!("  - {} ({})", name.bold(), path.display());
            }
            Err(anyhow::anyhow!("Profile '{}' not found", name))
        }
    }

    pub fn load_from_name_or_path(
        profile_name: Option<&str>,
        config_path: Option<&PathBuf>,
    ) -> Result<Option<Self>> {
        match (profile_name, config_path) {
            (Some(name), None) => {
                // Chercher le profil dans les dossiers possibles
                let profile_path = Self::find_profile(name)?;
                Ok(Some(Self::load(profile_path.to_str().unwrap())?))
            }
            (None, Some(path)) => Ok(Some(Self::load(path.to_str().unwrap())?)),
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

        let mut result = vec![[KeyCode::Null.to_u8(); 4]; 16];

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
            if binding.iter().any(|&k| k != KeyCode::Null.to_u8()) {
                let button_name = config::BUTTON_NAMES[i].to_string();
                let key_names: Vec<String> = binding
                    .iter()
                    .map(|&key| {
                        KeyCode::from_u8(key)
                            .map(|k| k.name())
                            .unwrap_or_else(|| format!("Unknown({:#04x})", key))
                    })
                    .collect();
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
        // Créer le répertoire parent si nécessaire
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }

        let toml = toml::to_string_pretty(&self)?;
        fs::write(path, toml)?;
        Ok(())
    }

    pub fn get_default_save_path(name: &str) -> PathBuf {
        if let Some(mut config_dir) = dirs::config_dir() {
            config_dir.push("8bitult");
            config_dir.push("profiles");
            config_dir.push(format!("{}.toml", name.to_lowercase().replace(' ', "_")));
            config_dir
        } else {
            PathBuf::from("profiles")
                .join(format!("{}.toml", name.to_lowercase().replace(' ', "_")))
        }
    }
}
