use std::{fs, io::Write};
use std::path::Path;
use sha1::{Sha1, Digest};
use hex::encode;

use crate::constants::VERCON_INIT_DIR;
use crate::hash::generate_dir_hash;

fn clear_staging() {
    let staging_dir_path = String::from("") + VERCON_INIT_DIR + "/staging";
    let _ = fs::remove_dir_all(&staging_dir_path);
    let _ = fs::create_dir(&staging_dir_path);
}

// Recursive function to read contents of route 
pub fn add_route(route: &str) {
    let mut hasher = Sha1::new();
    let path: &Path = Path::new(route);
    // Recurse if path is a directory
    if path.is_dir() {
        let read_dir_result = fs::read_dir(path).expect("Object at path is not a directory");
        for entry in read_dir_result {
            let path = entry.unwrap().path();
            add_route(path.to_str().unwrap());
        }
    }
    // Add hash if path is a file
    else if path.is_file() {
        let read_result = fs::read(path).expect("File does not exist");

        //  Create sha1 hash using contents of the file
        hasher.update(&read_result);
        let hash_result = hasher.finalize_reset();
        let hash= encode(hash_result);

        // Write file contents to staging 
        let _ = fs::write(String::from("") + VERCON_INIT_DIR + "/staging/" + hash.as_str(), read_result); 

        // append file name:file hash format
        let commit_info_route = String::from("") + VERCON_INIT_DIR + "/staging/commit_info";
        let commit_info_path = Path::new(commit_info_route.as_str());
        if commit_info_path.exists() {
            let mut stage_file = fs::OpenOptions::new()
                .append(true)
                .open(commit_info_path)
                .expect("Cannot open commit_info file");
            let _ = stage_file.write((String::from("") + path.to_str().unwrap() + ":" + hash.as_str() + "\n").as_bytes());

        }
        else {
            let _ = fs::write(commit_info_path.to_str().unwrap(), "");
        }
    }
    // Prolly wont happen
    else {
        println!("Path does not exist: {:?}", path.to_str())
    }
}

pub fn add(routes: Vec<String>) {
    let staging_dir_route = String::from("") + VERCON_INIT_DIR + "/staging";
    clear_staging();
    for route in routes {
        add_route(route.as_str());
    }
    let _ = fs::rename(staging_dir_route.clone() + "/commit_info", staging_dir_route.clone() + "/" + generate_dir_hash(staging_dir_route).as_str());
}