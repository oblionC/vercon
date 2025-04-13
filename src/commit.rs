use std::{fs, process::exit};
use std::io::Write;

use crate::hash::generate_dir_hash;
use crate::utils::path;
use crate::utils::constants::COMMIT_INFO_DELIMITER;

pub fn commit(message: &str) {
    if message.is_empty() {
        println!("You must provide a message");
        exit(1);
    }
    // Paths used
    let staging_dir_path = path::get_vercon_path("/staging/");
    let head_path = path::get_vercon_path("/HEAD");
    let objects_path =  path::get_vercon_path("/objects/");

    // Add message to commit info
    let commit_info_path = path::get_vercon_path("/commit_info"); 
    let commit_info_content = fs::read_to_string(&commit_info_path).expect("Could not read commit info file");
    if commit_info_content.is_empty() {
        println!("No files added");
        exit(1);
    }

    let mut commit_info_file = fs::OpenOptions::new()
    .append(true)
    .open(commit_info_path.clone())
    .expect("Could not open commit info file");
    let _ = commit_info_file.write(format!("{message}").as_bytes());

    let head_hash = fs::read_to_string(path::get_vercon_path("HEAD")).expect("Could not read HEAD");
    let _ = commit_info_file.write(format!("{COMMIT_INFO_DELIMITER}{head_hash}").as_bytes());

    // Read staging directory and create commit hash
    let hash = generate_dir_hash(staging_dir_path.clone()); 

    // Copy files from staging to commit directory
    let staging_dir = fs::read_dir(&staging_dir_path).unwrap();
    for file in staging_dir {
        let path = file.unwrap().path();
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