use nix::mount::*;
use nix::sys::wait::*;
use nix::unistd::*;
use nix::sched;
use std::ffi::CString;
use std::fs;

const ROOT_IMAGE_PATH: &str = "/media/deepankar/7039DF1C0BF3397F/Projects/coal/alpine/";
const STACK_SIZE: usize = 128 * 128;
const CGROUP_FOLDER: &str = "/sys/fs/cgroup/pids/coal/";


fn run_image(args: &Vec<String>) -> isize {
    limit_processes();
    // change to base image directory
    setup_root(ROOT_IMAGE_PATH);

    // mount proc
    mount::<str, str, str, str>(
        Some("proc"), "/proc", Some("proc"), MsFlags::MS_RDONLY, None,
    ).expect("mount error");

    // finally run the command in container
    let ref mut stack = [0; 64*64];
    sched::clone(
        Box::new(|| run(args.to_vec())), stack,
        sched::CloneFlags::empty(),
        Some(nix::sys::signal::Signal::SIGCHLD as i32)
    ).expect("Failed to spawn the command");
    wait().expect("wait failed for command");

    umount("/proc").expect("unmount failed");
    0
}

pub fn create(args: Vec<String>) {
    let ref mut stack = [0; STACK_SIZE];

    // clone the process
    let child_pid = sched::clone(
        Box::new(|| run_image(&args)),
        stack,
        sched::CloneFlags::CLONE_NEWPID | sched::CloneFlags::CLONE_NEWUTS,
        Some(nix::sys::signal::Signal::SIGCHLD as i32)
    ).expect("Failed to spawn the container");

    println!("child pid is {}", child_pid);

    match wait().expect("wait failed for container") {
        WaitStatus::Exited(_pid, _a) =>
            println!("child exited {}", _pid),
        _ => return,
    }
}

// util functions

fn setup_root(root_path: &str) {
    chroot(root_path).expect("error called");
    chdir("/").expect("change directory failed");
}

fn limit_processes() {    
    fs::create_dir(CGROUP_FOLDER).ok();
    let pid = getpid().as_raw().to_string();

    fs::write(format!("{}{}", CGROUP_FOLDER, "cgroup.procs"), pid).expect("Unable to write file");
    fs::write(format!("{}{}", CGROUP_FOLDER, "pids.max"), "5").expect("Unable to write file");
    fs::write(format!("{}{}", CGROUP_FOLDER, "notify_on_release"), "1").expect("Unable to write file");
}

fn run(args: Vec<String>) -> isize {
    let mut cargs: Vec<CString> = vec![];
    for arg in args {
        cargs.push(cstring(&arg));
    }

    execvp(&cargs[0], &cargs).expect("Invalid command to run");
    0
}

fn cstring(value: &str) -> CString {
    CString::new(value).expect("some err")
}