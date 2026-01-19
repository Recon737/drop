use droplet_rs::manifest::{ChunkData, Manifest};
use log::warn;

use crate::commands::upload::uploadable::Uploadable;

pub struct VoidUploadable;
impl Uploadable for VoidUploadable {
    fn upload_chunk(
        &mut self,
        _id: &String,
        _version: &String,
        _chunk_id: &String,
        _chunk: &ChunkData,
    ) -> anyhow::Result<()> {
        warn!("Uploading chunk to VoidUploader");
        Ok(())
    }

    fn upload_speedtest(&mut self, _game_id: &String, _version_id: &String) -> anyhow::Result<()> {
        warn!("Uploading speedtest to VoidUploader");
        Ok(())
    }

    fn upload_manifest(
        &mut self,
        _manifest: Manifest,
        _game_id: &String,
        _version_id: &String,
    ) -> anyhow::Result<()> {
        warn!("Uploading manifest to VoidUploader");
        Ok(())
    }
}
impl VoidUploadable {
    pub fn new() -> Self {
        Self
    }
}
