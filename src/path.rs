use crate::constants::VERCON_INIT_DIR;
use std::path::Path;

pub fn get(path: &str) -> String {
    let vercon_path_string = String::from("") + VERCON_INIT_DIR + "/" + path;
    let vercon_path= Path::new(vercon_path_string.as_str());  

    // If path is a dir, it needs a "/" at the end
    if vercon_path.is_file() {
        return vercon_path_string;
    }
    else {
        return vercon_path_string + "/";
    }
}