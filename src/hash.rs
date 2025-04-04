use std::{fs, path::Path};
use sha1::{Sha1, Digest};
use hex::encode;

// If the item is a directory, it updates the hasher with the hash value of the contents of the directory instead of its contents.
// May have to change it so that it uses the content in the future.
pub fn generate_dir_hash(route: String) -> String{
    let mut hasher = Sha1::new();
    let root_path: &Path = Path::new(route.as_str());
    let read_dir_result = fs::read_dir(root_path);

    match read_dir_result {     
        Ok(read_dir) => {
            for item in read_dir {
                let path = item.unwrap().path();
                if path.is_dir() {
                    let data = generate_dir_hash(String::from(path.to_str().unwrap()));
                    hasher.update(data);
                }
                else if path.is_file() {
                    hasher.update(fs::read(path).expect("Could not read file"));
                }
                else {
                    panic!("Path does not exist");
                }
            }
        },
        Err(e) => {
            if root_path.is_file() { 
                let data = fs::read(root_path).expect("Could not read file");
                hasher.update(data);
                return encode(hasher.finalize());
            }
            else {
                panic!("{e}");
            }
        }
    }
    return encode(hasher.finalize());
}