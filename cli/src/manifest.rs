use std::{
    path::Path,
};

use droplet_rs::manifest::{Manifest, generate_manifest_rusty};
use indicatif::{ProgressBar, ProgressStyle};

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
