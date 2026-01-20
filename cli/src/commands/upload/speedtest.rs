use rand::{RngCore, SeedableRng, rng, rngs::StdRng};
use tokio::io::AsyncRead;

#[derive(Clone, Debug)]
pub struct Speedtest {
    core: rand::rngs::StdRng,
    to_write: usize,
}
pub const SPEEDTEST_BYTES: usize = 64 * 1024 * 1024;
pub const SPEEDTEST_PATH: &str = "speedtest";

impl AsyncRead for Speedtest {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let mut s = self;
        let to_write = buf.remaining().min(s.to_write);
        s.to_write = s.to_write.saturating_sub(to_write);
        let fill_slice = buf.initialize_unfilled_to(to_write);
        s.core.fill_bytes(fill_slice);
        std::task::Poll::Ready(Ok(()))
    }
}
impl Speedtest {
    pub fn new() -> Self {
        Self {
            core: StdRng::from_rng(&mut rng()),
            to_write: SPEEDTEST_BYTES,
        }
    }
}
