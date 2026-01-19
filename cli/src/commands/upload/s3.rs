use crate::{
    commands::upload::{
        speedtest::{SPEEDTEST_PATH, Speedtest},
        uploadable::Uploadable,
    },
    config::S3Config,
};
use async_trait::async_trait;
use droplet_rs::manifest::{ChunkData, Manifest};
use s3::{Bucket, creds::Credentials};
use serde_json::json;
use std::{ops::Deref, path::PathBuf};

pub struct S3 {
    bucket: s3::Bucket,
}
impl S3 {
    pub fn new(config: &S3Config) -> anyhow::Result<Self> {
        Ok(Self {
            bucket: config.generate_bucket()?,
        })
    }
}
#[async_trait]
impl Uploadable for S3 {
    async fn upload_chunk(
        &mut self,
        id: &String,
        version: &String,
        chunk_id: &String,
        chunk: &ChunkData,
    ) -> anyhow::Result<()> {
        todo!()
    }
    async fn upload_speedtest(&mut self) -> anyhow::Result<()> {
        if self.object_exists(SPEEDTEST_PATH).await? {
            return Ok(());
        }
        let mut speedtest = Speedtest::new();
        self.put_object_stream(&mut speedtest, SPEEDTEST_PATH)
            .await?;
        Ok(())
    }

    async fn upload_manifest(
        &mut self,
        manifest: Manifest,
        game_id: &String,
        version_id: &String,
    ) -> anyhow::Result<()> {
        self.put_object(
            PathBuf::from(game_id)
                .join(version_id)
                .join("manifest.json")
                .to_string_lossy()
                .to_string(),
            json!(manifest).to_string().as_bytes(),
        )
        .await?;
        Ok(())
    }
}

impl Deref for S3 {
    type Target = Bucket;

    fn deref(&self) -> &Self::Target {
        &self.bucket
    }
}
