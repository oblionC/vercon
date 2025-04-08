mod init;
mod constants;
mod hash;
mod add;
mod commit;
mod path;

use std::{env, process::exit};

use hash::generate_dir_hash;

fn main() {
    // let _ = init::init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No arguments provided");
        exit(1);
    }
    
    let command = &args[1];
    if command == "init" {
        let _ = init::init();
    }
    else if command == "add" {
        let routes = &args[2..]; 
        println!("{:?}", routes);
        let _ = add::add(routes.to_vec());
    }
    else if command == "commit" {
        let _ = commit::commit();
    }
    else {
        println!("{}", generate_dir_hash("./test".to_string()));
        // panic!("Invalid command");
    }
}
