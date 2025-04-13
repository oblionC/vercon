use std::{collections::HashMap, fs,  path::PathBuf, process::exit};
use crate::{hash::generate_file_hash, utils::{constants::COMMIT_INFO_DELIMITER, path::{self, parse_route}}} ;
use strum_macros::FromRepr;


#[derive(FromRepr, Debug, PartialEq)]
enum CommitInfoSection {
    FilePaths=0,
    Message=1,
    PrevCommit=2,
}

#[derive(Debug)]
struct CommitInfo{
    file_paths: HashMap<PathBuf, String>,
    prev_commit: String,
    message: String, 
}

fn parse_commit_info(commit_info: &str, buf: &mut CommitInfo) {
    let split_commit_info = commit_info.split(COMMIT_INFO_DELIMITER);
    let mut file_paths_hashmap: HashMap<PathBuf, String>= HashMap::new();
    for (i, info) in split_commit_info.enumerate() {
        match CommitInfoSection::from_repr(i).unwrap() {
            CommitInfoSection::FilePaths => {
                let split_file_paths: Vec<String>= info.split("\n").filter(| file_path: &&str | !file_path.is_empty()).map(|s: &str| s.to_owned()).collect();
                for file_path in split_file_paths {
                    let key_val: Vec<&str> = file_path.split(":").collect();
                    file_paths_hashmap.insert(PathBuf::from(key_val[0]), key_val[1].to_string());
                }
            }
            CommitInfoSection::Message=> {
                buf.message = info.to_owned();
            }
            CommitInfoSection::PrevCommit=> {
                buf.prev_commit = info.to_owned();
            }
        }
    }
    buf.file_paths = file_paths_hashmap;
}

pub fn check_for_changes () {
    // Get head commit info
    let head_commit_hash = fs::read_to_string(path::get_vercon_path("HEAD")).expect("Could not read commit hash");
    if head_commit_hash.is_empty() {
        println!("No commits made");
        exit(1);
    }

    // Initialize commit info struct
    let mut commit_info_struct: CommitInfo = CommitInfo { 
        file_paths: HashMap::new(), 
        prev_commit: "".to_string(), 
        message: "".to_string() 
    };
    let commit_info_path = path::get_vercon_path(format!("objects/{}", head_commit_hash).as_str());
    let commit_info = fs::read_to_string(commit_info_path.clone()).expect("Could not get commit info");
    parse_commit_info(commit_info.as_str(), &mut commit_info_struct);

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