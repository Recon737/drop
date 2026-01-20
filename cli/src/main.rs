use crate::commands::configure::config::manage_configuration;
use crate::{
    cli::{Cli, Commands},
    commands::configure::config::Config,
    commands::upload,
};
use clap::Parser;
mod cli;
mod commands;
mod logging;
mod manifest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    crate::logging::configure_logging()?;

    let cli = Cli::parse();

    let mut config = Config::read();
    match &cli.command {
        Commands::Configure { name, option } => {
            manage_configuration(&mut config, name, option).await?
        }
        Commands::Upload(info) => {
            upload::interface::upload(info, config).await?;
        }
    };

    Ok(())
}
