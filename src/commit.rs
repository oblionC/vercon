use std::fs;

use crate::hash::generate_dir_hash;
use crate::path;

pub fn commit() {
    let staging_dir_path = path::get("/staging/");
    let head_path = path::get("/HEAD");
    let commit_info_path = path::get("/commit_info"); 
    let objects_path =  path::get("/objects/");

    // Read staging directory and create commit hash
    let hash = generate_dir_hash(staging_dir_path.clone()); 

    // Copy files from staging to commit directory
    let staging_dir = fs::read_dir(&staging_dir_path).unwrap();
    for file in staging_dir {
        let path = file.unwrap().path();
        println!("{:?}", path);
        let _ = fs::copy(&path, objects_path.clone() + &path.file_name().unwrap().to_str().unwrap()).expect("Could not copy file from staging dir: ");
    }

    // Move commit info to objects
    let _ = fs::copy(commit_info_path.clone(), objects_path + hash.as_str()).expect("Could not move commit info to objects");
    let _ = fs::write(commit_info_path, "");

    // Write commit hash to HEAD
    let _ = fs::write(head_path, hash.as_str());

    // Clear contents of staging directory
    let _ = fs::remove_dir_all(&staging_dir_path);
    let _ = fs::create_dir(&staging_dir_path);
}