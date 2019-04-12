use nix::sys::wait::*;
use nix::unistd::*;

pub fn create(command: &Fn(Vec<String>), args: Vec<String>) {
    match fork().expect("fork failed") {
        ForkResult::Parent{ child } => {
            match waitpid(child, None).expect("wait failed") {
                WaitStatus::Exited( _pid, _a) => println!("child exited {}", _pid),
                _ => return,
            }
        }
        ForkResult::Child => {
            command(args);
        }
    }
}