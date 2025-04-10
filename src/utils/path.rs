use std::{fs, path::Path, process::exit};
use crate::utils::constants::VERCON_INIT_DIR;

pub fn get_vercon_path(path: &str) -> String {
    let vercon_path_string = String::from("") + VERCON_INIT_DIR + "/" + path;
    let vercon_path= Path::new(vercon_path_string.as_str());  

    // If path is a dir, it needs a "/" at the end
    if vercon_path.is_dir() {
        return vercon_path_string + "/";
    }
    else {
        return vercon_path_string;
    }
}

fn push_path_to_buf(path: &str, buf: &mut Vec<String>) {
    let path_p = Path::new(path);
    if path_p.is_file() {
        buf.push(path.to_string());
    }
    else if path_p.is_dir() {
        let dir_contents = fs::read_dir(path).expect("Could not read directory");
        for item in dir_contents {
            push_path_to_buf(item.unwrap().path().to_str().unwrap(), buf);
        }
    }
    else {
        println!("Path does not exist");
        exit(1);
    }
}

pub fn get_all(path: &str) -> Vec<String> {
    let mut path_vec: Vec<String> = Vec::new();
    push_path_to_buf(path, &mut path_vec);
    path_vec
}