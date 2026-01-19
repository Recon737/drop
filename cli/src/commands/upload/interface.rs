use std::path::Path;

use crate::{
    cli::UploadInfo,
    commands::upload::{uploadable::Uploadable, void::VoidUploadable},
    manifest::generate_manifest,
};
use log::info;

pub async fn upload(info: &UploadInfo) -> anyhow::Result<()> {
    let game_id = &info.game_id;
    let path = &info.path;
    let version_id = &info.version_id;

    let manifest = generate_manifest(&Path::new(path)).await?;
    let mut uploader: Box<dyn Uploadable> = match info.upload_style {
        crate::cli::UploadStyle::S3 => Box::new(VoidUploadable::new()),
        crate::cli::UploadStyle::Nginx => Box::new(VoidUploadable::new()),
    };
    info!("Uploading chunks");
    for (id, data) in &manifest.chunks {
        info!("Uploading chunk id {id}");
        uploader.upload_chunk(game_id, version_id, id, data)?;
    }
    info!("Finished uploading chunks");

    info!("Uploading manifest");
    uploader.upload_manifest(manifest, game_id, version_id)?;

    info!("Uploading speedtest");
    uploader.upload_speedtest(game_id, version_id)?;

    Ok(())
}
