use std::{fs, process::exit, fmt};
use crate::{utils::path};
use strum_macros::FromRepr;

#[derive(FromRepr, Debug, PartialEq)]
enum CommitInfoSection {
    FilePaths=0,
    Message=1,
    PrevCommit=2,
}

#[derive(Debug)]
struct CommitInfo<'a>{
    file_paths: Vec<&'a str>,
    prev_commit: &'a str,
    message: &'a str, 
}

fn parse_commit_info(commit_info: String) {
    let split_commit_info = commit_info.split("-\n");
    let mut commit_info_struct: CommitInfo = CommitInfo { 
        file_paths: vec![], 
        prev_commit: "", 
        message: "" 
    };
    for (i, info) in split_commit_info.enumerate() {
        match CommitInfoSection::from_repr(i).unwrap() {
            CommitInfoSection::FilePaths => {
                let split_file_paths: Vec<&str>= info.split("\n").filter(| file_path: &&str | !file_path.is_empty()).collect();
                commit_info_struct.file_paths = split_file_paths;
            }
            CommitInfoSection::Message=> {
                commit_info_struct.message = info;
            }
            CommitInfoSection::PrevCommit=> {
                commit_info_struct.prev_commit = info;
            }
        }
    }
    println!("{:#?}", commit_info_struct);
}

pub fn check_for_changes (path: String) {
    // Get head commit info
    let head_commit_hash = fs::read_to_string(path::get_vercon_path("HEAD")).expect("Could not read commit hash");
    if head_commit_hash.is_empty() {
        println!("No commits made");
        exit(1);
    }
    let commit_info_path = path::get_vercon_path(format!("objects/{}", head_commit_hash).as_str());
    let commit_info = fs::read_to_string(commit_info_path.clone()).expect("Could not get commit info");
    parse_commit_info(commit_info);

    // Get all items in root dir
    // let paths_vec = path::get_all(path.as_str());
    // for p in paths_vec {
    //     println!("{}", p);
    // }

}