extern crate nix;

use nix::unistd::sleep;
use std::env::args;

mod container;

const COMMAND: &str = "run";

fn main() {
    // for arg in args() {
    //     println!("arg: {:?}", arg);
    // }

    let args_list: Vec<String> = args().collect();
    if args_list.len() > 1 && args_list[1] == COMMAND {
        container::create(&run);
    } else {
        println!("please type in correct command");
    }
}

fn run() {
    println!("run called");
    for _a in 0..3 {
        println!("child running");
        sleep(1);
    }
}
