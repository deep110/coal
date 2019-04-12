use nix::sys::wait::*;
use nix::unistd::*;

const ROOT_IMAGE_PATH: &str = "/media/deepankar/7039DF1C0BF3397F/Projects/coal/alpine/";


fn setup_root(root_path: &str) {
    chroot(root_path).expect("error called");
    chdir("/").expect("change directory failed");
}

pub fn create(command: &Fn(Vec<String>), args: Vec<String>) {
    match fork().expect("fork failed") {
        ForkResult::Parent{ child } => {
            match waitpid(child, None).expect("wait failed") {
                WaitStatus::Exited( _pid, _a) => println!("child exited {}", _pid),
                _ => return,
            }
        }
        ForkResult::Child => {
            setup_root(ROOT_IMAGE_PATH);
            command(args);
        }
    }
}