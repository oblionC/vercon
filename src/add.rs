use std::fs::{self, DirEntry};
use sha1::{Sha1, Digest};
use std::path::Path;

pub fn add_route(route: &str) {
    let mut hasher = Sha1::new();
    let path: &Path = Path::new(route);
    if path.is_dir() {
        let read_dir_result = fs::read_dir(path).expect("Object at path is not a directory");
        for dir in read_dir_result {
            let path = dir.unwrap().path();
            println!("{}", path.to_str().unwrap());
            add_route(path.to_str().unwrap());
        }
    }
    else if path.is_file() {
        let read_result = fs::read(path).expect("File does not exist");
        hasher.update(read_result);
        let result = hasher.finalize_reset();
    }
    else {
        println!("Path does not exist")
    }
}

fn add(routes: Vec<String>) {
    unimplemented!()
}