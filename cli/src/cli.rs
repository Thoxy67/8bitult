use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all available profiles
    List,

    /// Read current bindings from device
    Read,

    /// Attach a profile to the device
    Attach {
        /// Profile name to load from profiles directory
        #[arg(short = 'p', long = "profile", group = "input")]
        profile_name: Option<String>,

        /// Path to a specific TOML profile file
        #[arg(short = 'c', long = "config", group = "input")]
        config_file: Option<PathBuf>,
    },
}
