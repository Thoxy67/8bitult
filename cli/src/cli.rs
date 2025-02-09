use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Verbosity level (-v = info, -vv = debug, -vvv = trace)
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    pub verbose: u8,

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

    /// Save current device configuration as a new profile
    Save {
        /// Name of the new profile
        #[arg(short = 'n', long = "name")]
        name: String,

        /// Description of the profile
        #[arg(short = 'd', long = "description")]
        description: Option<String>,

        /// Save path (defaults to user config directory)
        #[arg(short = 'o', long = "output")]
        output: Option<PathBuf>,
    },

    /// Backup profiles management
    Backup {
        /// Import profiles from archive
        #[arg(long = "import", group = "action")]
        import: bool,

        /// Export profiles to archive
        #[arg(long = "export", group = "action")]
        export: bool,

        /// Archive path for export
        #[arg(long = "save", requires = "export")]
        save: Option<PathBuf>,
    },
}
