extern crate nix;

use std::env::args;

mod container;

const COMMAND: &str = "run";

fn main() {
    let args_list: Vec<String> = args().collect();
    if args_list.len() > 1 && args_list[1] == COMMAND {
        container::create(args_list[2..].to_vec());
    } else {
        println!("please type in correct command");
    }
}
