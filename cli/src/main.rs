use crate::commands::configure::config_option::ConfigOptionCli;
use crate::commands::configure::configure::Configurable;
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
        Commands::Configure { name, option } => config.add_item(
            name.clone(),
            match option {
                ConfigOptionCli::Server(server_config) => server_config.clone().configure().await?,
                ConfigOptionCli::S3(s3_config_cli) => s3_config_cli.clone().configure().await?,
            },
        ),
        Commands::Upload(info) => {
            upload::interface::upload(info, config).await?;
        }
    };

    Ok(())
}
