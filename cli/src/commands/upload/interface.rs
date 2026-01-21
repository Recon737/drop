use std::path::{Path, PathBuf};

use crate::{
    cli::UploadInfo,
    commands::configure::config::Config,
    commands::upload::{s3::S3, uploadable::Uploadable},
    manifest::generate_manifest,
};
use log::info;

pub async fn upload(info: &UploadInfo, config: Config) -> anyhow::Result<()> {
    let game_id = &info.game_id;
    let path = &info.path;
    let version_id = &info.version_id;

    let manifest = generate_manifest(&Path::new(path)).await?;
    let mut uploader: Box<dyn Uploadable> = match info.upload_style {
        crate::cli::UploadStyle::S3 => Box::new(S3::new(
            &config
                .get_active_s3()
                .ok_or(anyhow::Error::msg("Could not get active S3 value"))?,
        )?),
    };
    info!("Uploading chunks");
    for (id, data) in &manifest.chunks {
        info!("Uploading chunk id {id}");
        uploader.upload_chunk(PathBuf::from(path), game_id, version_id, id, data).await?;
    }
    info!("Finished uploading chunks");

    info!("Uploading manifest");
    uploader
        .upload_manifest(manifest, game_id, version_id)
        .await?;

    info!("Uploading speedtest");
    uploader.upload_speedtest().await?;

    Ok(())
}
