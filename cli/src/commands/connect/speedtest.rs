use rand::{RngCore, SeedableRng, rng, rngs::StdRng};
use tokio::io::AsyncRead;

#[derive(Clone, Debug)]
pub struct Speedtest<F: Fn(f32)> {
    core: rand::rngs::StdRng,
    to_write: usize,
    callback: Box<F>,
}
pub const SPEEDTEST_BYTES: usize = 64 * 1024 * 1024;
pub const SPEEDTEST_PATH: &str = "speedtest";

impl<F: Fn(f32)> AsyncRead for Speedtest<F> {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let mut s = self;
        let to_write = buf.remaining().min(s.to_write);

        let filled = {
            let fill_slice = buf.initialize_unfilled_to(to_write);
            s.core.fill_bytes(fill_slice);
            fill_slice.len()
        };
        s.to_write = s.to_write.saturating_sub(filled);
        (s.callback)((1f32 - (s.to_write as f32 / SPEEDTEST_BYTES as f32)) * 100f32);
        buf.advance(filled);
        std::task::Poll::Ready(Ok(()))
    }
}
impl<F: Fn(f32)> Speedtest<F> {
    pub fn new(callback: F) -> Self {
        Self {
            core: StdRng::from_rng(&mut rng()),
            to_write: SPEEDTEST_BYTES,
            callback: Box::new(callback),
        }
    }
}
