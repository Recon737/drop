use async_trait::async_trait;
use droplet_rs::manifest::{ChunkData, Manifest};

#[async_trait]
pub trait Uploadable {
    async fn upload_chunk(
        &mut self,
        id: &String,
        version: &String,
        chunk_id: &String,
        chunk: &ChunkData,
    ) -> anyhow::Result<()>;
    async fn upload_speedtest(&mut self) -> anyhow::Result<()>;
    async fn upload_manifest(
        &mut self,
        manifest: Manifest,
        game_id: &String,
        version_id: &String,
    ) -> anyhow::Result<()>;
}
