use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Args, Clone)]
pub struct ServerConfig {
    /// Endpoint of the Drop server
    url: String,
    #[arg(short, long)]
    token: String,
}