mod init;
mod add;
mod commit;
mod utils;
mod status;
mod diff;
mod log;

use std::{env, process::exit};
use utils::path::validate_route;

fn main() {
    // let _ = init::init();
    validate_route("xdd");
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
        let _ = add::add(routes.to_vec());
    }
    else if command == "commit" {
        if args.len() > 2 {
            let message = &args[2]; 
            let _ = commit::commit(message.as_str());
        }
        else {
            println!("You must provide a message"); 
            exit(1);
        }
    }
    else if command == "status" {
        let _ = status::check_for_changes();
    }
    else if command == "diff" {
        let route = &args[2]; 
        diff::print_diff(route.to_owned());
    }
    else if command == "log" {
        log::log();
    }
    else {
        panic!("Invalid command");
    }
}
