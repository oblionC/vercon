use std::{fs, io::Write};
use std::path::Path;
use sha1::{Sha1, Digest};
use hex::encode;

use crate::constants::VERCON_INIT_DIR;
use crate::hash::generate_dir_hash;

pub fn commit() {
    let mut hasher = Sha1::new();
    let staging_dir_path = String::from("") + VERCON_INIT_DIR + "/staging";

    // Read staging directory and create commit hash
    let hash = encode(hasher.finalize_reset());

    // Write commit hash to HEAD
    let _ = fs::write(String::from("") + VERCON_INIT_DIR + "/HEAD", hash.as_str());

    // Append commit hash to commit info
    let mut stage_file = fs::OpenOptions::new()
        .append(true)
        .open(String::from("") + VERCON_INIT_DIR + "/staging/commit_info")
        .expect("Cannot open file");
    let _ = stage_file.write(hash.as_bytes());

    // Create commit directory in current branch 
    // TODO: Change branch when branching is added
    let _ = fs::create_dir(String::from("") + VERCON_INIT_DIR + "/branches/main/" + hash.as_str());

    // Copy files from staging to commit directory
    let staging_dir = fs::read_dir(&staging_dir_path).unwrap();
    for file in staging_dir {
        let path = file.unwrap().path();
        println!("{:?}", path);
        let _ = fs::copy(&path, String::from("") + VERCON_INIT_DIR + "/branches/main/" + hash.as_str() + "/" + &path.file_name().unwrap().to_str().unwrap()).expect("Could not copy file from staging dir: ");
    }

    // Clear contents of staging directory
    let _ = fs::remove_dir_all(&staging_dir_path);
    let _ = fs::create_dir(&staging_dir_path);
}