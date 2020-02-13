pub extern crate libc;
pub extern crate nix;

use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use libc::{prctl, PR_SET_CHILD_SUBREAPER};
use nix::sys::wait::waitpid;
use nix::unistd::{fork, getpid, getppid, ForkResult, Pid};

fn main()
{
    println!("[Main] main here.");
    println!("[Main] I'll become a child subreaper!");
    unsafe {
        prctl(PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0);
    }
    match fork() {
        Ok(ForkResult::Child) => {
            println!("[Child1] I'm born! My PID is {} and my PPID is {}", getpid(), getppid());
            
            match fork() {
                Ok(ForkResult::Child) => {
                    println!("[Child2] I'm born! My PID is {} and my PPID is {}", getpid(), getppid());
                    for _ in 0..6 {
                        println!("[Child2] sleeping...");
                        sleep(Duration::from_millis(500));
                    }
                    println!("[Child2] exiting...");
                    exit(0);
                }
                Ok(ForkResult::Parent { child, .. }) => {
                    println!("[Child1] I forked a child with PID {}", child);
                }
                Err(e) => panic!("[Child1] fork() failed with error code {}...", e),
            };
            println!("[Child1] I'm gonna sleep then exit()!");
            sleep(Duration::from_millis(1500));
            exit(0);
        }
        Ok(ForkResult::Parent { child, .. }) => {
            println!("[Main] I forked a child with PID {} and PPID {}!", child, getpid());
        }
        Err(e) => panic!("[Main] fork() failed with error code {}...", e),
    }
    println!("[Main] I'll be waiting for the child termination...");
    match waitpid(Pid::from_raw(-1), None) {
        Ok(status) => println!("[Main] Child returned with status {:?}!", status),
        Err(e) => panic!("[Main] Child returned with error code {}...", e),
    };

    println!("[Main] I'll be waiting for the grandchild termination too...");
    sleep(Duration::from_millis(500));
    match waitpid(Pid::from_raw(-1), None) {
        Ok(status) => println!("[Main] Grandchild returned with status {:?}!", status),
        Err(e) => panic!("[Main] Grandchild returned with error code {}...", e),
    };
    println!("[Main] Good bye!");
}