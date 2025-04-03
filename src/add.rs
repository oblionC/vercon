use std::{fs, io::Write};
use std::path::Path;
use sha1::{Sha1, Digest};
use hex::encode;

use crate::constants::VERCON_INIT_DIR;

fn clear_stage_info() {
    let _ = fs::write(String::from("") + VERCON_INIT_DIR + "/staging/commit_info", "");
}

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
        let result = hasher.finalize_reset();
        let result = encode(result);

        let _ = fs::write(String::from("") + VERCON_INIT_DIR + "/staging/" + result.as_str(), read_result); 

        let mut stage_file = fs::OpenOptions::new()
            .append(true)
            .open(String::from("") + VERCON_INIT_DIR + "/staging/commit_info")
            .expect("Cannot open file");
        let _ = stage_file.write((String::from("") + path.to_str().unwrap() + ":" + result.as_str() + "\n").as_bytes());
    }
    // Prolly wont happen
    else {
        println!("Path does not exist: {:?}", path.to_str())
    }
}

pub fn add(routes: Vec<String>) {
    clear_stage_info();
    for route in routes {
        add_route(route.as_str());
    }
}