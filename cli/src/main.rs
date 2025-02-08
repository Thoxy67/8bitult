use heigtbitult::{config, keyboard, BleKeyboard};
use clap::Parser;
use colored::Colorize;
use profile::Profile;
use tokio::time::sleep;
use tracing::{instrument::WithSubscriber, Level};
use core::time;
use std::{error::Error, path::PathBuf};
use tabled::{settings::{object::Rows, themes::Colorization, Alignment, Color, Style}, Table, Tabled};


#[derive(Tabled)]
struct KeyBindingRow {
    #[tabled(rename = "Touche")]
    button: String,
    #[tabled(rename = "1")]
    key1: String,
    #[tabled(rename = "2")]
    key2: String,
    #[tabled(rename = "3")]
    key3: String,
    #[tabled(rename = "4")]
    key4: String,
}

mod profile;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the TOML profile
    #[arg(short, long)]
    profile: Option<PathBuf>,

    /// Read current bindings without modifying them
    #[arg(short, long)]
    read: bool,
}

fn print_section_header(text: &str) {
    println!("\n{}", "━".repeat(50).bright_black());
    println!("{}", text.bold());
    println!("{}", "━".repeat(50).bright_black());
}

fn print_step(text: &str) {
    println!("\n{} {}", "→".bright_cyan(), text);
}

fn print_bindings(bindings: &[[u8; 4]], button_names: &[&str]) {
    let rows: Vec<KeyBindingRow> = bindings.iter().enumerate()
        .map(|(i, binding)| {
            let key_names: Vec<String> = binding
                .iter()
                .map(|&key| {
                    let name = keyboard::get_key_name(key);
                    if name == "NULL" {
                        "-".to_string()
                    } else {
                        name
                    }
                })
                .collect();

            KeyBindingRow {
                button: button_names[i].to_string(),
                key1: key_names[0].clone(),
                key2: key_names[1].clone(),
                key3: key_names[2].clone(),
                key4: key_names[3].clone(),
            }
        })
        .collect();

    let mut table = Table::new(rows);
    table
        .with(Style::modern())
        .with(Colorization::columns([
            Color::FG_BRIGHT_WHITE,
            Color::FG_RED,
            Color::FG_GREEN,
            Color::FG_BLUE,
            Color::FG_YELLOW,
        ]))
        .modify(Rows::first(), Alignment::center());

    println!("\n{}", table);
}

fn print_success(text: &str) {
    println!("{} {}", "✓".green(), text.green());
}

fn print_warning(text: &str) {
    println!("{} {}", "!".yellow(), text.yellow());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG)
        // sets this to be the default, global collector for this application.
        .init();
    let cli = Cli::parse();
    
    print_section_header("8Bitdo Micro Configurator");
    
    print_step("Searching for device...");
    let mut keyboard = BleKeyboard::new().await?;
    print_success("Device connected successfully!");
    
    print_step("Reading current bindings...");
    let current_bindings = keyboard.read_current_bindings().await?;
    println!("\nCurrent bindings configuration:");
    print_bindings(&current_bindings, &config::BUTTON_NAMES);

    if !cli.read {
        if let Some(profile_path) = cli.profile {
            print_step("Loading profile...");
            let profile = Profile::load(profile_path.to_str().unwrap())?;
            print_success(&format!("Loaded profile: '{}'", profile.name));
            
            let new_bindings = profile.to_key_bindings()?;
            
            print_step("Writing new bindings...");
            keyboard.write_bindings(&new_bindings).await?;
            print_success("Bindings written successfully");
            
            sleep(time::Duration::from_secs(2)).await;

            print_step("Verifying new bindings...");
            let updated_bindings = keyboard.read_current_bindings().await?;
            
            // Vérifier si les bindings correspondent
            let matches = new_bindings.iter().zip(updated_bindings.iter()).all(|(a, b)| a == b);
            
            println!("\nNew bindings configuration:");
            print_bindings(&updated_bindings, &config::BUTTON_NAMES);

            if !matches {
                print_warning("Warning: Some bindings might not have been applied correctly");
            } else {
                print_success("All bindings verified successfully");
            }
        } else {
            print_warning("No profile specified, skipping write");
        }
    }

    print_step("Disconnecting device...");
    keyboard.disconnect().await?;
    print_success("Device disconnected successfully");
    
    Ok(())
}