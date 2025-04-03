use std::{fs, io::Write};
use std::path::Path;
use sha1::{Sha1, Digest};
use hex::encode;

use crate::constants::VERCON_INIT_DIR;

pub fn commit() {
    let mut hasher = Sha1::new();
    let staging_dir_path = String::from("") + VERCON_INIT_DIR + "/staging";

    let read_dir_result = fs::read_dir(&staging_dir_path);
    for file in read_dir_result.expect("Could not read file") {
        let path = file.unwrap().path();
        let content = fs::read(path).expect("Could not read file ");
        hasher.update(content);
    }
    let hash = encode(hasher.finalize_reset());

    let _ = fs::write(String::from("") + VERCON_INIT_DIR + "/HEAD", hash.as_str());
    let _ = fs::create_dir(String::from("") + VERCON_INIT_DIR + "/branches/main/" + hash.as_str());

    let staging_dir = fs::read_dir(&staging_dir_path).unwrap();
    for file in staging_dir {
        let path = file.unwrap().path();
        println!("{:?}", path);
        let _ = fs::copy(&path, String::from("") + VERCON_INIT_DIR + "/branches/main/" + hash.as_str() + "/" + &path.file_name().unwrap().to_str().unwrap()).expect("Could not copy file from staging dir: ");
    }
    let _ = fs::remove_dir_all(&staging_dir_path);
    let _ = fs::create_dir(&staging_dir_path);
}