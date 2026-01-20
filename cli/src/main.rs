use crate::config::configure::Configurable;
use crate::{
    cli::{Cli, Commands},
    commands::upload,
    config::config::Config,
};
use clap::Parser;
mod cli;
mod commands;
mod config;
mod logging;
mod manifest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    crate::logging::configure_logging()?;

    let cli = Cli::parse();

    let mut config = Config::read();
    match &cli.command {
        Commands::Configure { name, option } => match option {
            config::config::ConfigOptionCli::Server(server_config) => todo!(),
            config::config::ConfigOptionCli::S3(s3_config_cli) => config.add_item(
                name.clone(),
                config::config::ConfigOption::S3(s3_config_cli.clone().configure()),
            ),
        },
        Commands::Upload(info) => {
            upload::interface::upload(info, config).await?;
        }
    };

    Ok(())
}
