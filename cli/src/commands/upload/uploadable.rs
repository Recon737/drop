use droplet_rs::manifest::{ChunkData, Manifest};

pub trait Uploadable {
    fn upload_chunk(
        &mut self,
        id: &String,
        version: &String,
        chunk_id: &String,
        chunk: &ChunkData,
    ) -> anyhow::Result<()>;
    fn upload_speedtest(&mut self, game_id: &String, version_id: &String) -> anyhow::Result<()>;
    fn upload_manifest(&mut self, manifest: Manifest, game_id: &String, version_id: &String) -> anyhow::Result<()>;
}
pub enum UploadableConfig {
    S3 {
        api_secret: String,
        api_key_identifier: String,
        region: String,
    },
}
