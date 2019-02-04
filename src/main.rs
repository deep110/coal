extern crate nix;

use std::env::args;

mod container;


const COMMANDS:&[&str]  = &["run"];

fn main() {
    // for arg in args() {
    //     println!("arg: {:?}", arg);
    // }

    let args_list: Vec<String> = args().collect();
    if args_list.len() > 1 && COMMANDS.contains(&args_list[1].as_ref()) {
        run_command(&args_list[1].as_ref());
    } else {
        invalid_command();
    }
}

fn run_command(command: &str) {
    match command {
        "run" => run(),
        _ => invalid_command(),
    };
}

fn invalid_command() {
    println!("please type in correct command");
}

fn run() {
    println!("run called");
    container::test();
}
