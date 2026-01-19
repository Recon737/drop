use droplet_rs::manifest::{ChunkData, Manifest};

use crate::commands::upload::uploadable::Uploadable;

pub type S3 = aws_sdk_s3::Client;
impl Uploadable for S3 {
    fn upload_chunk(
        &mut self,
        id: &String,
        version: &String,
        chunk_id: &String,
        chunk: &ChunkData,
    ) -> anyhow::Result<()> {
        todo!()
    }
    fn upload_speedtest(&mut self, game_id: &String, version_id: &String) -> anyhow::Result<()> {
        todo!()
    }

    fn upload_manifest(
        &mut self,
        manifest: Manifest,
        game_id: &String,
        version_id: &String,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
