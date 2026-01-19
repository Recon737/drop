use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

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
        token: Option<String>,
    },
    /// Uploads new game version to depot
    Upload(UploadInfo),
}
#[derive(Args)]
pub struct UploadInfo {
    /// Identifies the specific upload style that will be used for the set depot
    pub upload_style: UploadStyle,
    /// Relative path to new version files
    #[arg(short, long)]
    pub path: PathBuf,
    /// ID of game to attach to
    #[arg(short, long)]
    pub game_id: String,
    /// Version ID to attach to
    #[arg(short, long)]
    pub version_id: String,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum UploadStyle {
    S3,
}
