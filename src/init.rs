use std::{fs, fs::File, process::exit};
use crate::utils::path;

fn handle_create_dir_err(dir_path: String) {
    let create_dir_result = fs::create_dir(&dir_path);
    match &create_dir_result{
        Ok(()) => {
            println!("{dir_path} directory created")
        },
        Err(e) => {
            println!("Error while creating directory: {e}");
            exit(1)
        }
    }
}

fn handle_create_file_error(file_path: String) -> Result<File, std::io::Error> {
    let create_file_error: Result<File, std::io::Error> = File::create(&file_path);
    match &create_file_error {
        Ok(_) => {
            println!("{file_path} directory created");
        },
        Err(e) => {
            println!("Error while creating file: {e}");
        }
    }
    create_file_error
}

fn create_vercon_dir(dir_path: &str) {
    handle_create_dir_err(path::get_vercon_path(dir_path));
}

fn create_vercon_file(file_path: &str) {
    println!("{}", path::get_vercon_path(file_path));
    let _ = handle_create_file_error(path::get_vercon_path(file_path));
}

pub fn init() -> std::io::Result<()>{
    // Generate all directories
    // Create parent directory before the child directory 
    create_vercon_dir("");

    create_vercon_dir("/branches");
    create_vercon_dir("/branches/main");

    create_vercon_dir("/staging");
    create_vercon_dir("/objects");

    create_vercon_file("/HEAD");
    create_vercon_file("/ignore");
    create_vercon_file("/commit_info");
    
    Ok(())
}