use clap::Args;
use serde::{Deserialize, Serialize};

use crate::config::configurable::Configurable;

#[derive(Serialize, Deserialize, Args, Clone)]
pub struct ServerConfig {
    /// Endpoint of the Drop server
    url: String,
    #[arg(short, long)]
    token: String,
}

impl Configurable for ServerConfig {
    fn configure(&self, config: &mut super::config::Config) {
        println!("Configured ServerConfig")
    }
}