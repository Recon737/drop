use std::path::PathBuf;

use droplet_rs::manifest::generate_manifest_rusty;
use tokio::runtime::Handle;

#[tokio::main]
pub async fn main() {
    let metrics = Handle::current().metrics();
    println!("using {} workers", metrics.num_workers());
    generate_manifest_rusty(
        &PathBuf::from("/home/decduck/.local/share/Steam/steamapps/common/Savage Resurrection"),
        |progress| {
            println!("PROGRESS: {}", progress)
        },
        |message| {
            println!("{}", message);
        },
    )
    .await
    .unwrap();
}
