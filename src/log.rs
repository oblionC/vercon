use std::fs;
use crate::{commit, utils::{parse::{parse_commit_info, CommitInfo}, path::get_vercon_path}};

pub fn log() {
    let head_hash = fs::read_to_string(get_vercon_path("HEAD"));
    if head_hash.is_ok() {
        let mut current_hash = head_hash.unwrap();
        while !current_hash.is_empty() {
            let mut commit_info = CommitInfo::new();
            parse_commit_info(current_hash.to_owned(), &mut commit_info);
            println!("---");
            println!("Commit: {current_hash}");
            println!("Message: {}", commit_info.message);
            println!("Date: {}", commit_info.date);
            println!("---");
            current_hash = commit_info.prev_commit;
        }
    }
}