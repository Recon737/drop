use std::path::Path;

use crate::{
    cli::UploadInfo,
    commands::{
        connect::config::Config,
        upload::chunk_reader::ChunkReader,
    },
    manifest::generate_manifest, operator_builder::OperatorBuilder,
};
use futures::AsyncWriteExt;
use log::info;
use tokio_util::compat::FuturesAsyncWriteCompatExt;

pub async fn upload(info: &UploadInfo, config: Config) -> anyhow::Result<()> {
    let game_id = &info.game_id;
    let path = &info.path;
    let version_id = &info.version_id;

    let manifest = generate_manifest(Path::new(path)).await?;
    let operator = match info.upload_style {
        crate::cli::UploadStyle::S3 => config
            .get_active_s3()
            .ok_or(anyhow::Error::msg("Could not get active S3 value"))?
            .build()?,
    };
    info!("Uploading chunks");
    for (id, data) in &manifest.chunks {
        info!("Uploading chunk id {id}");
        let mut reader = ChunkReader::new(path, data);
        let mut writer = operator
            .writer(&format!("{game_id}/{version_id}/{id}"))
            .await?
            .into_futures_async_write()
            .compat_write();
        tokio::io::copy(&mut reader, &mut writer).await?;
        writer.into_inner().close().await?;
    }
    info!("Finished uploading chunks");
    Ok(())
}
