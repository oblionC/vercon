use std::{collections::HashMap, fs,  path::PathBuf};
use crate::utils::{ constants::COMMIT_INFO_DELIMITER, path } ;
use strum_macros::FromRepr;

#[derive(FromRepr, Debug, PartialEq)]
pub enum CommitInfoSection {
    FilePaths=0,
    Message=1,
    PrevCommit=2,
    Date=3,
}

#[derive(Debug)]
pub struct CommitInfo{
    pub file_paths: HashMap<PathBuf, String>,
    pub prev_commit: String,
    pub message: String, 
    pub date: String, 
}

impl CommitInfo {
    pub fn new() -> CommitInfo {
        let commit_info_struct: CommitInfo = CommitInfo { 
            file_paths: HashMap::new(), 
            prev_commit: "".to_string(), 
            message: "".to_string(), 
            date: "".to_string(), 
        };
        commit_info_struct
    }
}

pub fn parse_commit_info(commit_hash: String, buf: &mut CommitInfo) {
    let commit_info_path = path::get_vercon_path(format!("objects/{}", commit_hash).as_str());
    let commit_info = fs::read_to_string(commit_info_path.clone()).expect("Could not get commit info");
    let split_commit_info = commit_info.split(COMMIT_INFO_DELIMITER);
    let mut file_paths_hashmap: HashMap<PathBuf, String>= HashMap::new();
    for (i, info) in split_commit_info.enumerate() {
        match CommitInfoSection::from_repr(i).unwrap() {
            CommitInfoSection::FilePaths => {
                let split_file_paths: Vec<String>= info.split("\n").filter(| file_path: &&str | !file_path.is_empty()).map(|s: &str| s.to_owned()).collect();

                for file_path in split_file_paths {
                    let key_val: Vec<&str> = file_path.split(":").collect();
                    if key_val.len() >= 2 {
                        file_paths_hashmap.insert(PathBuf::from(key_val[0]), key_val[1].to_string());
                    }
                }
            }

            CommitInfoSection::Message=> {
                buf.message = info.to_owned();
            }
            CommitInfoSection::PrevCommit=> {
                buf.prev_commit = info.to_owned();
            }
            CommitInfoSection::Date => {
                buf.date = info.to_owned();
            }
        }
    }
    buf.file_paths = file_paths_hashmap;
}
