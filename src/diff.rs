use std::fs;

use similar::{ChangeTag, TextDiff};
use crate::utils::{hash::get_head_hash, parse::{parse_commit_info, CommitInfo}, path::{get_vercon_path, parse_route}};

pub fn print_diff(file: String) {
    println!("{file}");
    let file_path = parse_route(file.as_str());
    // Get contents of both versions of the file: HEAD commit version and the current version
    let current_file_content = fs::read_to_string(file.to_owned()).expect("Could not read file");

    let head_commit_hash = get_head_hash();
    let mut commit_info_struct = CommitInfo::new();
    parse_commit_info(head_commit_hash, &mut commit_info_struct);
    let file_hash = commit_info_struct.file_paths.get(&parse_route(file.as_str())).expect("File not found in commit info");
    let old_file_content = fs::read_to_string(get_vercon_path(format!("objects/{}", file_hash).as_str())).expect("Could not read old file contents");

    let diff = TextDiff::from_lines(&old_file_content, &current_file_content);

    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => {
                "-"
            },
            ChangeTag::Insert => {
                "+"
            },
            ChangeTag::Equal => {
                ""
            },
        };
        let old_index = change.old_index().map_or(String::new(), |x| x.to_string());
        let new_index = change.new_index().map_or(String::new(), |x| x.to_string());

        print!("{:03}{:03} |{} {}", old_index, new_index, sign, change)
    }
}