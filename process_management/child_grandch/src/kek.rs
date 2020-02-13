use std::ffi::{c_void};
use std::thread;
use std::time::Duration;

use libc::{prctl, PR_SET_PDEATHSIG, STDOUT_FILENO, write};
use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, SIGUSR1};
use nix::unistd::{getpid, getppid, Pid};

extern "C" fn handle_sigusr1(_: libc::c_int) {
    print_signal_safe("[Kek] Parent terminated!");
}


fn main() {
    println!("[Kek] Hey! MY PID is {}", getpid());
    unsafe {
        prctl(PR_SET_PDEATHSIG, SIGUSR1);
    }
    let sig_action = SigAction::new(SigHandler::Handler(handle_sigusr1), SaFlags::empty(), SigSet::empty(),);
    if let Err(e) = unsafe {
        sigaction(SIGUSR1, &sig_action)
    } {
            println!("[Kek] sigaction() failed with error code {}...", e);
    };
    loop {
        let ppid = getppid();
        println!("[Kek] my parent is {}!", ppid);
        thread::sleep(Duration::from_millis(500));
        if ppid == Pid::from_raw(1) {
            println!("[Kek] My parent is init process!");
            break;
        }
    }
    println!("[Kek] Bye kek!");
}

fn print_signal_safe(s: &str)
{
    unsafe {
        write(STDOUT_FILENO, s.as_ptr() as *const c_void, s.len());
    }
}
