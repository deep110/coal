use nix::sys::wait::*;
use nix::unistd::*;

pub fn create(command: &Fn()) {
    match fork().expect("fork failed") {
        ForkResult::Parent{ child } => {
            match waitpid(child, None).expect("wait failed") {
                WaitStatus::Exited( _pid, _a) => println!("child exited {}", _pid),
                _ => return,
            }
        }
        ForkResult::Child => {
            command();
        }
    }
}