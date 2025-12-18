use std::{
    collections::HashMap,
    future::Future,
    path::Path,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use anyhow::{anyhow, Error};
use futures::{stream::FuturesUnordered, StreamExt};
use hex::ToHex as _;
use humansize::{format_size, BINARY};
use serde::Serialize;
use serde_json::json;
use sha2::{Digest as _, Sha256};
use tokio::{io::AsyncReadExt as _, join, sync::Mutex};

#[derive(Serialize, Clone)]
pub struct FileEntry {
    filename: String,
    start: usize,
    length: usize,
    permissions: u32,
}

#[derive(Serialize, Clone)]
pub struct ChunkData {
    files: Vec<FileEntry>,
    checksum: String,
    //iv: [u8; 16]
}

#[derive(Serialize)]
pub struct Manifest {
    version: String,
    chunks: HashMap<String, ChunkData>,
    size: u64,
}

const CHUNK_SIZE: u64 = 1024 * 1024 * 64;
const WIGGLE: u64 = 1024 * 1024;

use crate::versions::{create_backend_constructor, types::VersionFile};

pub async fn generate_manifest_rusty<T: Fn(String), V: Fn(f32)>(
    dir: &Path,
    progress_sfn: V,
    log_sfn: T,
) -> anyhow::Result<Manifest> {
    let mut backend =
        create_backend_constructor(dir).ok_or(anyhow!("Could not create backend for path."))?()?;

    let required_single_file = backend.require_whole_files();

    let files = backend.list_files().await?;
    // Filepath to chunk data
    let mut chunks: Vec<Vec<(VersionFile, u64, u64)>> = Vec::new();
    let mut current_chunk: Vec<(VersionFile, u64, u64)> = Vec::new();

    log_sfn("organizing files into chunks...".to_string());

    for version_file in files {
        // If we need the whole file, and this file would take up a whole chunk, add it to it's own chunk and move on
        if required_single_file && version_file.size >= CHUNK_SIZE {
            let size = version_file.size;
            chunks.push(vec![(version_file, 0, size)]);

            continue;
        }

        let mut current_size = current_chunk.iter().map(|v| v.2 - v.1).sum::<u64>();

        // If we need the whole file, add this current file and move on, potentially adding and creating new chunk if need be
        if required_single_file {
            let size = version_file.size;
            current_chunk.push((version_file, 0, size));

            current_size += size;

            if current_size >= CHUNK_SIZE {
                // Pop current and add, then reset
                let new_chunk = std::mem::take(&mut current_chunk);
                chunks.push(new_chunk);
            }

            continue;
        }

        // Otherwise we calculate how much of the file we need, then use that much
        let remaining_budget = (CHUNK_SIZE + WIGGLE) - current_size;
        if version_file.size >= remaining_budget {
            let remaining_budget = CHUNK_SIZE - current_size;
            current_chunk.push((version_file.clone(), 0, remaining_budget));

            let new_chunk = std::mem::take(&mut current_chunk);
            chunks.push(new_chunk);

            let remaining_size = version_file.size - remaining_budget;
            let mut running_offset = remaining_budget;
            // Do everything but the last one
            while running_offset < remaining_size {
                let chunk_size = CHUNK_SIZE.min(remaining_size);
                let chunk = vec![(version_file.clone(), running_offset, chunk_size)];
                if chunk_size == CHUNK_SIZE {
                    chunks.push(chunk);
                } else {
                    current_chunk = chunk;
                }
                running_offset += chunk_size;
            }

            continue;
        } else {
            let size = version_file.size;
            current_chunk.push((version_file, 0, size));
            current_size += size;
        }

        if current_size >= CHUNK_SIZE {
            // Pop current and add, then reset
            let new_chunk = std::mem::take(&mut current_chunk);
            chunks.push(new_chunk);
        }
    }
    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    log_sfn(format!(
        "organized into {} chunks, generating checksums...",
        chunks.len()
    ));

    let manifest: Arc<Mutex<HashMap<String, ChunkData>>> = Arc::new(Mutex::new(HashMap::new()));
    let total_manifest_length = Arc::new(AtomicU64::new(0));

    let backend = Arc::new(Mutex::new(backend));

    let futures: FuturesUnordered<impl Future<Output = Result<(), Error>>> =
        FuturesUnordered::new();
    let (send_log, mut recieve_log) = tokio::sync::mpsc::channel(16);
    let chunks_length = chunks.len();
    for (index, chunk) in chunks.into_iter().enumerate() {
        let send_log = send_log.clone();
        let backend = backend.clone();
        let total_manifest_length = total_manifest_length.clone();
        let manifest = manifest.clone();
        futures.push(async move {
            let mut read_buf = vec![0; 1024 * 1024 * 64];

            let uuid = uuid::Uuid::new_v4().to_string();
            let mut hasher = Sha256::new();

            let mut chunk_data = ChunkData {
                files: Vec::new(),
                checksum: String::new(),
                //iv: 
            };

            let mut chunk_length = 0;

            for (file, start, length) in chunk {
                let mut reader = {
                    let mut backend_lock = backend.lock().await;
                    let reader = backend_lock.reader(&file, start, start + length).await?;
                    reader
                };

                loop {
                    let amount = reader.read(&mut read_buf).await?;
                    if amount == 0 {
                        break;
                    }
                    hasher.update(&read_buf[0..amount]);
                }

                chunk_length += length;

                chunk_data.files.push(FileEntry {
                    filename: file.relative_filename,
                    start: start.try_into().unwrap(),
                    length: length.try_into().unwrap(),
                    permissions: file.permission,
                });
            }

            send_log
                .send(format!(
                    "created chunk of size {} from {} files (index {})",
                    format_size(chunk_length, BINARY),
                    chunk_data.files.len(),
                    index
                ))
                .await?;

            total_manifest_length.fetch_add(chunk_length, Ordering::Relaxed);

            let hash: String = hasher.finalize().encode_hex();
            chunk_data.checksum = hash;
            {
                let mut manifest_lock = manifest.lock().await;
                manifest_lock.insert(uuid, chunk_data);
            };

            Ok(())
        });
    }
    drop(send_log);
    join!(
        async move {
            let mut current_progress = 0f32;
            let total_progress = chunks_length as f32;
            while let Some(message) = recieve_log.recv().await {
                log_sfn(message);
                current_progress += 1.0f32;
                progress_sfn((current_progress / total_progress) * 100.0f32)
            }
        },
        futures.collect::<Vec<Result<(), Error>>>()
    );

    let manifest = manifest.lock().await;
    let manifest = manifest.clone();

    Ok(Manifest {
        version: "2".to_string(),
        chunks: manifest,
        size: total_manifest_length.fetch_add(0, Ordering::Relaxed)
    })
}
