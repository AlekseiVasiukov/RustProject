// build.rs
use fs_extra::dir::{self, CopyOptions};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Tell cargo to rerun this script if the assets folder changes.
    println!("cargo:rerun-if-changed=assets");

    // Get the output directory for the final executable.
    // CARGO_TARGET_DIR is not set by default, so we build the path from the manifest dir.
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap(); // "debug" or "release"
    let target_dir = Path::new(&manifest_dir).join("target").join(profile);

    // Define the source and destination paths for the assets.
    let source_assets_path = Path::new(&manifest_dir).join("assets");

    // Create the destination assets directory if it doesn't exist.
    if !target_dir.exists() {
        fs::create_dir_all(&target_dir).expect("Failed to create assets directory in target.");
    }

    let options = CopyOptions::new().overwrite(true);

    dir::copy(&source_assets_path, &target_dir, &options)
        .expect("Failed to copy directory");

}