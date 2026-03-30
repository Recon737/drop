use clap::Subcommand;
use opendal::{Operator, layers::LoggingLayer};
use serde::{Deserialize, Serialize};

use crate::{
    commands::connect::s3::{S3Config, S3ConfigCli},
    operator_builder::OperatorBuilder,
};

#[derive(Subcommand, Clone)]
pub enum ConfigOptionCli {
    // Connect to any S3-compatible endpoint
    S3(S3ConfigCli),
}
#[derive(Serialize, Deserialize, Clone)]
pub enum ConfigOption {
    S3(S3Config),
}

impl ConfigOption {
    pub fn build(&self) -> anyhow::Result<Operator> {
        Ok(match self {
            ConfigOption::S3(s3_config) => s3_config.build()?,
        }
        .layer(LoggingLayer::default()))
    }
}
