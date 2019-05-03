use nix::mount::*;
use nix::sys::wait::*;
use nix::unistd::*;
use nix::sched;
use std::ffi::CString;

const ROOT_IMAGE_PATH: &str = "/media/deepankar/7039DF1C0BF3397F/Projects/coal/alpine/";
const STACK_SIZE: usize = 512 * 512;


fn run_image(args: &Vec<String>) -> isize {
    print!("run image called");
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
    run(args.to_vec());

    umount("/proc").expect("unmount failed");
    0
}

pub fn create(args: Vec<String>) {
    let ref mut stack = [0; STACK_SIZE];

    // clone the process
    let child_pid = sched::clone(
        Box::new(|| run_image(&args)),
        stack,
        sched::CloneFlags::CLONE_NEWUTS,
        Some(nix::sys::signal::Signal::SIGCHLD as i32)
    ).expect("Failed to spawn the child");

    println!("child pid is {}", child_pid);

    match wait().expect("wait failed") {
        WaitStatus::Exited(_pid, _a) =>
            println!("child exited {}", _pid),
        _ => return,
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
