use crate::{
    cli::{Cli, Commands},
    commands::{configure::interactive_configure, upload},
};
use clap::Parser;
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use std::env;
use std::fs;
use std::io;

mod cli;
mod commands;
mod manifest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    configure_logging()?;
    let cli = Cli::parse();
    match &cli.command {
        Commands::Configure { url, token } => {
            if let Some(token) = token {
            } else {
                interactive_configure(url.to_string()).await?;
            }
        }
        Commands::Upload(info) => {
            upload::interface::upload(info).await?;
        }
    };

    Ok(())
}

pub fn configure_logging() -> anyhow::Result<()> {
    let log_level = env::var("RUST_LOG")
        .or_else(|_| env::var("LOG_LEVEL"))
        .unwrap_or_else(|_| "info".to_string())
        .parse::<LevelFilter>()?;

    let log_dir = env::var("LOG_FILE_DIR").unwrap_or_else(|_| "logs".to_string());

    fs::create_dir_all(&log_dir)?;

    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::Magenta);

    fern::Dispatch::new()
        .chain(
            // Console output with colors and formatting
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "[{}] {} {} - {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                        colors.color(record.level()),
                        record.target(),
                        message
                    ))
                })
                .chain(io::stdout()),
        )
        .chain(
            // File output without colors and with formatting
            fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{}] {} {} - {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                        record.level(),
                        record.target(),
                        message
                    ))
                })
                .chain(fern::log_file(format!("{}/app.log", log_dir))?),
        )
        .level(log_level)
        .apply()?;

    Ok(())
}
