use std::process::exit;
use crate::utils::path::{self, parse_route} ;
use crate::utils::hash::*;
use crate::utils::parse::{CommitInfo, parse_commit_info};


pub fn check_for_changes () {
    // Get head commit info
    let head_commit_hash = get_head_hash();
    if head_commit_hash.is_empty() {
        println!("No commits made");
        exit(1);
    }

    // Initialize commit info struct
    let mut commit_info_struct = CommitInfo::new(); 
    parse_commit_info(head_commit_hash, &mut commit_info_struct);

    let all_files = path::get_all("./test");

    // All changed files
    println!("Changes to files: ");
    for file in all_files {
        let file_path = parse_route(&file);
        if file_path.exists() {
            let prev_hash = commit_info_struct.file_paths.get(&file_path).unwrap().to_owned();
            let new_hash = generate_file_hash(file.to_owned());

            if prev_hash != new_hash {
                println!("\t{:?}", file_path);
            }
            else {
            }
        }
    }

    // All deleted files
    println!("Deleted files: ");
    for file in commit_info_struct.file_paths.into_keys() {
        if !file.exists() {
            println!("\t{:?}", file);
        }
        else {
        }
    }
}