use droplet_rs::manifest::ChunkData;
use std::{
    cmp::min,
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
    task::Poll,
    vec::IntoIter,
};
use tokio::io::AsyncRead;

pub struct ChunkReader {
    files: IntoIter<LimitedFileReader>,
    active: Option<LimitedFileReader>,
}

pub struct LimitedFileReader {
    file: File,
    to_read_remaining: usize,
}

impl Read for LimitedFileReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let to_read = min(self.to_read_remaining, buf.len());
        let (to_read, _remaining) = buf.split_at_mut(to_read);
        let read = self.file.read(to_read)?;
        self.to_read_remaining -= read;
        Ok(read)
    }
}

impl ChunkReader {
    pub fn new(path: impl AsRef<Path>, chunk: &ChunkData) -> Self {
        let files = chunk
            .files
            .iter()
            .map(|f| {
                let mut file = File::open(path.as_ref().join(&f.filename)).unwrap();
                file.seek(SeekFrom::Start(f.start as u64)).unwrap(); // TODO: Fix unwraps
                LimitedFileReader {
                    file,
                    to_read_remaining: f.length,
                }
            })
            .collect::<Vec<LimitedFileReader>>()
            .into_iter();
        Self {
            files,
            active: None,
        }
    }
}
impl AsyncRead for ChunkReader {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let mut s = self;
        loop {
            if let Some(active) = &mut s.active {
                match active.read(buf.initialize_unfilled()) {
                    Ok(0) => {
                        s.active = None;
                        continue;
                    }
                    Ok(n) => {
                        buf.advance(n);

                        return Poll::Ready(Ok(()));
                    }
                    Err(e) => return Poll::Ready(Err(e)),
                }
            } else {
                if let Some(next) = s.files.next() {
                    s.active = Some(next);
                } else {
                    return Poll::Ready(Ok(()));
                }
            }
        }
    }
}

// impl Read for ChunkReader {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         loop {
//             if let Some(active) = &mut self.active {
//                 match active.read(buf) {
//                     Ok(0) => {
//                         self.active = None;
//                         continue;
//                     }
//                     Ok(n) => return Ok(n),
//                     Err(e) => return Err(e),
//                 }
//             } else {
//                 if let Some(next) = self.files.next() {
//                     self.active = Some(next);
//                 } else {
//                     return Ok(0);
//                 }
//             }
//         }
//     }
// }
