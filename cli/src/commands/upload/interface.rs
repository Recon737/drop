use std::{io::SeekFrom, path::Path, pin::Pin};

use crate::{
    cli::UploadInfo,
    commands::connect::{config::Config, config_option::ConfigOption},
    manifest::{CompressionOption, DepotManifest, generate_v2_manifest},
    operator_builder::OperatorBuilder,
};
use droplet_rs::manifest::ChunkData;
use futures::{AsyncWriteExt, StreamExt, TryStreamExt, future::join_all, stream};
use log::info;
use opendal::Operator;
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncReadExt, AsyncSeekExt, Take},
};
use tokio_util::{
    bytes::Bytes,
    compat::FuturesAsyncWriteCompatExt,
    io::{ReaderStream, StreamReader},
};

pub async fn upload(
    info: &UploadInfo,
    config: Config,
    name: &Option<String>,
) -> anyhow::Result<()> {
    let game_id = &info.game_id;
    let path = &info.path;
    let version_id = &info.version_id;

    let operator = get_operator(config, name)?;

    let mut existing_depot_manifest = get_depot_manifest(&operator).await?;

    let v2_manifest = generate_v2_manifest(Path::new(path)).await?;

    info!("Uploading chunks");

    for (id, data) in &v2_manifest.chunks {
        info!("Uploading chunk id {id}");
        let mut reader = generate_chunk_readstream(path, data).await;
        let mut writer = operator
            .writer(&format!("{game_id}/{version_id}/{id}"))
            .await?
            .into_futures_async_write()
            .compat_write();
        tokio::io::copy(&mut reader, &mut writer).await?;
        writer.into_inner().close().await?;
    }

    info!("Finished uploading chunks");

    existing_depot_manifest.append(
        game_id.to_string(),
        version_id.to_string(),
        CompressionOption::None,
    );
    Ok(())
}

// Black magic don't touch
/// Connects all of the files at the correct start and end points into a single, continuous AsyncRead object
pub async fn generate_chunk_readstream<'a, P: AsRef<Path> + 'a>(
    path: P,
    data: &'a ChunkData,
) -> Pin<Box<impl AsyncRead>> {
    let path = path.as_ref().to_path_buf();
    let files = data.files.clone();

    let stream = stream::iter(files)
        .map(move |f| {
            let path = path.clone();
            // Lazy block to ensure that not too many files get opened at once
            async move {
                let mut file = File::open(path.join(f.filename)).await?;
                file.seek(SeekFrom::Start(f.start as u64)).await?;
                tokio::io::Result::Ok(file.take(f.length as u64))
            }
        })
        .buffered(2) // Could also be 1. Just removes a bit of latency from opening files buy preparing the next one immediately
        .map_ok(|file| ReaderStream::new(file))
        .try_flatten();
    let reader = StreamReader::new(stream);
    Box::pin(reader)
}

async fn get_depot_manifest(operator: &Operator) -> Result<DepotManifest, anyhow::Error> {
    let existing_depot_manifest = operator.read("manifest.json").await?.to_bytes();
    let existing_depot_manifest: DepotManifest =
        serde_json::from_slice(existing_depot_manifest.as_ref())?;
    Ok(existing_depot_manifest)
}

fn get_operator(config: Config, name: &Option<String>) -> anyhow::Result<Operator> {
    let operator = match if let Some(name) = name {
        config
            .get(name)
            .ok_or(anyhow::anyhow!("Name does not exist"))?
    } else {
        config.get_active().ok_or(anyhow::anyhow!(
            "No active connection set. Please specify with --name"
        ))?
    } {
        ConfigOption::S3(s3_config) => s3_config.build()?,
    };
    Ok(operator)
}
