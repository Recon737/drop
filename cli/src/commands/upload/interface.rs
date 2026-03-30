use std::path::Path;

use crate::{
    cli::UploadInfo,
    commands::connect::{config::Config, config_option::ConfigOption},
    manifest::{CompressionOption, DepotManifest, generate_v2_manifest},
    operator_builder::OperatorBuilder,
};
use futures::AsyncWriteExt;
use log::info;
use opendal::{FuturesAsyncWriter, Operator};
use tokio_util::compat::{Compat, FuturesAsyncWriteCompatExt};

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

    info!("Uploading chunks");

    let v2_manifest = generate_v2_manifest(
        Path::new(path),
        async |id: String| {
            info!("Uploading chunk id {id}");
            let writer = operator
                .writer(&format!("{game_id}/{version_id}/{id}"))
                .await
                .unwrap()
                .into_futures_async_write()
                .compat_write();
            writer
        },
        |writer: Compat<FuturesAsyncWriter>| async {
            writer.into_inner().close().await.unwrap();
        },
    )
    .await?;

    info!("Finished uploading chunks");

    existing_depot_manifest.append(
        game_id.to_string(),
        version_id.to_string(),
        CompressionOption::None,
    );
    Ok(())
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
