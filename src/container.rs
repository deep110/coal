use nix::mount::*;
use nix::sys::wait::*;
use nix::unistd::*;
use std::ffi::CString;

const ROOT_IMAGE_PATH: &str = "/media/deepankar/7039DF1C0BF3397F/Projects/coal/alpine/";

fn run_image(args: Vec<String>) {
    // change to base image directory
    setup_root(ROOT_IMAGE_PATH);

    // mount proc
    mount::<str, str, str, str>(
        Some("proc"),
        "/proc",
        Some("proc"),
        MsFlags::MS_REMOUNT,
        None,
    ).expect("mount error");

    // finally run the command in container
    run(args);

    umount("/proc").expect("unmount failed");
}

pub fn create(args: Vec<String>) {
    match fork().expect("fork failed") {
        ForkResult::Parent { child } => match waitpid(child, None).expect("wait failed") {
            WaitStatus::Exited(_pid, _a) => println!("child exited {}", _pid),
            _ => return,
        },
        ForkResult::Child => {
            run_image(args);
        }
    }
}

// util functions

fn cstring(value: &str) -> CString {
    CString::new(value).expect("some err")
}

fn setup_root(root_path: &str) {
    chroot(root_path).expect("error called");
    chdir("/").expect("change directory failed");
}

fn run(args: Vec<String>) {
    let mut cargs: Vec<CString> = vec![];
    for arg in args {
        cargs.push(cstring(&arg));
    }

    execvp(&cargs[0], &cargs).expect("Invalid command to run");
}