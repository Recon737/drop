use std::{env, path::PathBuf};

use droplet_rs::manifest::generate_manifest_rusty;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::runtime::Handle;

#[tokio::main]
async fn main() {
    let threads = Handle::current().metrics().num_workers();
    println!("using {} workers", threads);
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let path = PathBuf::from(path);
    if !path.exists() {
        panic!("{} does not exist", path.display());
    }

    println!("using path {}", path.display());

    let progress_bar = ProgressBar::new(100_00).with_style(
        ProgressStyle::template(
            ProgressStyle::default_bar(),
            "[{elapsed_precise}] [ETA {eta}] {wide_bar} {percent_precise}%",
        )
        .unwrap(),
    );
    let manifest = generate_manifest_rusty(
        &path,
        |progress| {
            let progress_int = (progress * 100.0f32).round() as u64;
            progress_bar.set_position(progress_int);
        },
        |log| {
            progress_bar.println(log);
        },
    )
    .await
    .expect("failed to generate manifest");
}
