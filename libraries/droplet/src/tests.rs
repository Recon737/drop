#![cfg(test)]
extern crate test_generator;

use std::path::Path;

use serde_json::json;
use test_generator::test_resources;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::manifest::generate_manifest_rusty;

#[test_resources("testfiles/**/*.7z")]
fn manifest_gen(resource: &str) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to create tokio runtime");

    runtime.block_on(async move {
        let filepath = Path::new(resource);
        let manifest = generate_manifest_rusty(
            &filepath,
            |_| {},
            |message| {
                println!("({}) {}", filepath.display(), message);
            },
            None,
        )
        .await
        .expect(&format!(
            "failed to generate manifest for {}",
            filepath.display()
        ));

        let mut output_path = filepath.to_path_buf();
        output_path.set_extension("json");

        let mut file = File::create(output_path)
            .await
            .expect("failed to open output path");
        file.write_all(json!(manifest).to_string().as_bytes())
            .await
            .expect("failed to write output");
    });
}
