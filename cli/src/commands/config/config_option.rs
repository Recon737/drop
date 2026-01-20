use clap::Subcommand;
use serde::{Deserialize, Serialize};

use crate::commands::config::{
    s3::{S3Config, S3ConfigCli},
    server::{ServerConfig, ServerConfigCli},
};

#[derive(Subcommand, Clone)]
pub enum ConfigOptionCli {
    Server(ServerConfigCli),
    S3(S3ConfigCli),
}
#[derive(Serialize, Deserialize, Clone)]
pub enum ConfigOption {
    Server(ServerConfig),
    S3(S3Config),
}
