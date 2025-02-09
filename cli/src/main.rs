mod cli;
mod commands;
mod profile;
mod ui;

use clap::Parser;
use tracing::Level;

fn get_log_level(verbosity: u8) -> Level {
    match verbosity {
        0 => Level::WARN,
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    tracing_subscriber::fmt()
        .with_max_level(get_log_level(cli.verbose))
        .init();

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
        cli::Commands::Save { name, output } => {
            commands::handle_save(name, output).await?;
        }
    }

    Ok(())
}
