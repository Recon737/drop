use crate::commands::{
    configure::s3::S3Config,
    upload::{
        chunk_reader::ChunkReader,
        speedtest::{SPEEDTEST_PATH, Speedtest},
        uploadable::Uploadable,
    },
};
use async_trait::async_trait;
use droplet_rs::{
    manifest::{ChunkData, Manifest},
};
use s3::Bucket;
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
        base_path: PathBuf,
        id: &String,
        version: &String,
        chunk_id: &String,
        chunk: &ChunkData,
    ) -> anyhow::Result<()> {
        let path = &PathBuf::from(id)
            .join(version)
            .join(chunk_id)
            .to_string_lossy()
            .to_string();
        let mut reader = ChunkReader::new(&base_path, chunk);
        self.put_object_stream(&mut reader, &path).await?;
        Ok(())
    }
    async fn upload_speedtest(&mut self) -> anyhow::Result<()> {
        if self.object_exists(SPEEDTEST_PATH).await? {
            return Ok(());
        }
        println!("Uploading speedtest");
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
        self.put_object_builder(
            PathBuf::from(game_id)
                .join(version_id)
                .join("manifest.json")
                .to_string_lossy()
                .to_string(),
            json!(manifest).to_string().as_bytes(),
        )
        .with_content_type("application/json")
        .execute()
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
