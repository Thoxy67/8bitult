mod cli;
mod commands;
mod profile;
mod ui;

use clap::Parser;
use tracing::Level;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let cli = cli::Cli::parse();
    ui::print_section_header("8Bitdo Micro Configurator");

    match cli.command {
        cli::Commands::List => {
            commands::handle_list().await?;
        }
        cli::Commands::Read => {
            commands::handle_read().await?;
        }
        cli::Commands::Attach {
            profile_name,
            config_file,
        } => {
            commands::handle_attach(profile_name, config_file).await?;
        }
    }

    Ok(())
}
