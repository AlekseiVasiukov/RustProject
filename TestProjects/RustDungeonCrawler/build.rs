// build.rs
use fs_extra::dir::{self, CopyOptions};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Tell cargo to rerun this script if the assets folder changes.
    println!("cargo:rerun-if-changed=src/assets");

    // Get the output directory for the final executable.
    // CARGO_TARGET_DIR is not set by default, so we build the path from the manifest dir.
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap(); // "debug" or "release"
    let target_dir = Path::new(&manifest_dir).join("target").join(profile);

    // Define the source and destination paths for the assets.
    let source_assets_path = Path::new(&manifest_dir).join("src").join("assets");
    let dest_assets_path = target_dir;

    // Create the destination assets directory if it doesn't exist.
    if !dest_assets_path.exists() {
        fs::create_dir_all(&dest_assets_path).expect("Failed to create assets directory in target.");
    }

    // Copy the TOML file.
    // This can be expanded to copy all files in the assets directory if needed.
    let source_toml = source_assets_path.join("Weapon_Bases.toml");
    let dest_toml = dest_assets_path.join("Weapon_Bases.toml");

    fs::copy(&source_toml, &dest_toml)
        .expect(&format!("Failed to copy {} to {}", source_toml.display(), dest_toml.display()));
    
    let options = CopyOptions::new().overwrite(true);

    dir::copy(&source_assets_path, &dest_assets_path, &options)
        .expect("Failed to copy directory");

    // We no longer need to set the WEAPON_BASES_PATH environment variable.
}