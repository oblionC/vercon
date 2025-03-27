use std::{fs, process::exit};
use crate::constants::VERCON_INIT_DIR;

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

pub fn init() -> std::io::Result<()>{
    let vercon_init_dir = String::from(VERCON_INIT_DIR);

    // Generate all directories
    // Create parent directory before the child directory 
    handle_create_dir_err(String::from("") + &vercon_init_dir);

    handle_create_dir_err(String::from("") + &vercon_init_dir + &"/branches".to_string());
    handle_create_dir_err(String::from("") + &vercon_init_dir + &"/branches/main".to_string());

    handle_create_dir_err(String::from("") + &vercon_init_dir + &"/base".to_string());
    handle_create_dir_err(String::from("") + &vercon_init_dir + &"/main".to_string());
    handle_create_dir_err(String::from("") + &vercon_init_dir + &"/staging".to_string());
    
    Ok(())
}