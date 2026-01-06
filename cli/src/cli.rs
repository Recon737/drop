use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Specify data file path
    #[arg(short, long)]
    pub data: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Configures a new Drop server
    Configure {
        /// Endpoint of the Drop server
        url: String,
        /// API token for non-interactive configuration.
        #[arg(short, long)]
        token: Option<String>
    },
    /// Uploads new game version to depot
    Upload {
        /// Path of new version
        path: bool,
        /// ID of game to attach to
        game_id: String,
        /// Version ID to attach to
        version_id: String,
    },
}
