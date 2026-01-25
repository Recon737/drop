use std::{collections::HashMap, path::Path};

use droplet_rs::manifest::{Manifest, generate_manifest_rusty};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DepotManifest {
    content: HashMap<String, DepotManifestGameData>,
}
#[derive(Serialize, Deserialize)]
struct DepotManifestGameData {
    version_id: String,
    compression: CompressionOption,
}
#[derive(Serialize, Deserialize)]
pub enum CompressionOption {
    None,
    Gzip,
    Zstd,
}
impl DepotManifest {
    pub fn new() -> Self {
        Self {
            content: HashMap::new(),
        }
    }
    pub fn add(&mut self, game_id: String, version_id: String, compression: CompressionOption) {
        self.content.insert(
            game_id,
            DepotManifestGameData {
                version_id,
                compression,
            },
        );
    }
}

pub async fn generate_manifest(dir: &Path) -> anyhow::Result<Manifest> {
    let progress_bar = ProgressBar::new(100_00).with_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [ETA {eta}] {bar} {percent_precise}%")
            .unwrap(),
    );
    let res = generate_manifest_rusty(
        dir,
        |progress| {
            let progress_int = (progress * 100f32).round() as u64;
            progress_bar.set_position(progress_int);
        },
        |log| progress_bar.println(log),
    )
    .await;
    res
}
