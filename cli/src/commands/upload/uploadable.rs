use std::path::PathBuf;

use async_trait::async_trait;
use droplet_rs::manifest::{ChunkData, Manifest};
use opendal::Operator;

#[async_trait]
pub trait OperatorBuilder {
    fn build(&self) -> anyhow::Result<Operator>;
}
