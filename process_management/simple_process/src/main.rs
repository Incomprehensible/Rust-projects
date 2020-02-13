pub extern crate libc;

use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use libc::c_void;
use libc::sigaction;
use nix::sys::signal::sigaction;

use nix::sys::wait::waitpid;
use nix::unistd::{fork, getpid, getppid, ForkResult};
use libc::{_exit, STDOUT_FILENO, write};

extern "C" fn handle_sigchld(_: libc::c_int) {
    print_signal_safe("[Main] Got SIGCHILD!\n");
    match waitpid(Pid::from_raw(-1), None) {
        Ok(_) => {
            print_signal_safe("[Main] Child exited.\n");
            exit_signal_safe(0);
        }
        Err(_) => {
            print_signal_safe("[Main] waitpid() failed...\n");
            exit_signal_safe(1);
        }
    }
}

fn main() {
    println!("[Main] Hey! My PID is {}!", getpid());

    let child_pid = match fork() {
        Ok(ForkResult::Child) => {
            println!("[Child] I'm born! My PID is {}!", getpid());
            println!("[Child] Also, my PPID is {}!", getppid());
            println!("I'll sleep and exit()");
            sleep(Duration::from_secs(2));
            exit(0);
        }
        Ok(ForkResult::Parent {child, ..}) => {
            println!("[Main] I forked a child with PID {}!", child);
            child
        }
        Err(err) => {
            panic!("[Main] fork() failed with error code {}...", err);
        }
    };
    let sig_action = SigAction::new(
        SigHandler::Handler(handle_sigchld), SaFlags::empty(), SigSet::empty(),
    );
    if let Err(e) = unsafe { sigaction(SIGCHILD, &sig_action)} {
        panic!("[Main] sigaction() failed with error code {}", e);
    };
    loop {
        println!("[Main] waiting for child termination...");
    }
    match waitpid(child_pid, None) {
        Ok(status) => println!("[Main] Child returned with status {:?}!", status),
        Err(e) => panic!("[Main] Child returned with error code {}...", e),
    };
    println!("[Main] Good bye!");
}

fn print_signal_safe(s: &str)
{
    unsafe {
        write(STDOUT_FILENO, s.as_ptr() as (*const c_void), s.len());
    }
}

fn exit_signal_safe(status: i32)
{
    unsafe {
        _exit(status);
    }
}
