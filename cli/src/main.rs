use std::{env, path::PathBuf, sync::LazyLock};

use anyhow::Result;
use clap::Parser;
use droplet_rs::manifest::generate_manifest_rusty;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::runtime::Handle;

use crate::{cli::{Cli, Commands}, commands::configure::interactive_configure};

mod cli;
mod commands;

pub static CLI: LazyLock<Cli> = LazyLock::new(|| Cli::parse());

#[tokio::main]
async fn main() -> Result<()> {

    match &CLI.command {
        Commands::Configure { url, token } => {
            if let Some(token) = token {
                todo!()
            } else {
                interactive_configure(url.to_string()).await?;
            }
        },
        Commands::Upload {
            path,
            game_id,
            version_id,
        } => todo!(),
    };

    Ok(())
}
