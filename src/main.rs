mod init;
mod constants;
mod add;
mod commit;

use std::{env, process::exit};

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
        panic!("Invalid command");
    }
}
