extern crate nix;

use nix::unistd::*;
use std::env::args;
use std::ffi::CString;

mod container;
mod utils;

const COMMAND: &str = "run";

fn main() {
    let args_list: Vec<String> = args().collect();
    if args_list.len() > 1 && args_list[1] == COMMAND {
        container::create(&run, args_list[2..].to_vec());
    } else {
        println!("please type in correct command");
    }
}

fn run(args: Vec<String>) {
    let mut cargs: Vec<CString> = vec![];
    for arg in args {
        cargs.push(utils::cstring(&arg));
    }

    execvp(&cargs[0], &cargs).expect("Invalid command to run");
}
