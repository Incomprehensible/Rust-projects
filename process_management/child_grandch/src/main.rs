use std::ffi::CString;
use std::process::{exit};
use std::thread::{sleep};
use std::time::Duration;

use libc::{prctl, PR_SET_CHILD_SUBREAPER};
use nix::sys::wait::waitpid;
use nix::unistd::{execv, fork, ForkResult, getpid, getppid, Pid};

fn main()
{
    println!("[Main] Hey! I'm main. My PID is {}!", getpid());
    unsafe {
        prctl(PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0);
    }

    match fork() {
        Ok(ForkResult::Parent { child, .. }) => println!("[Main] Forked a child with pid {}!", child),
        Ok(ForkResult::Child) => {
            println!("[Child] Hey! I'm born and my PID is {}, my PPID is {}!", getpid(), getppid());

            match fork() {
                Ok(ForkResult::Child) =>
                {
                    println!("[Grandchild] I'm born too! My PID is {}, my PPID is {}!", getpid(), getppid());
                    println!("Grandchild] I'll execute another process and will be reborn!");
                    exec_or_die("target/debug/kek");
                }
                Ok(ForkResult::Parent { child, .. }) => println!("[Child] I forked my own child with PID {}!", child),
                Err(e) => panic!("[Child] fork() failed with error code {}...", e),
            }
            println!("[Child] I'll sleep, then die...");
            sleep(Duration::from_millis(1500));
            exit(0);
        }
        Err(e) => panic!("[Main] fork() failed with error code {}...", e),
    };
    println!("[Main] Waiting for child termination!");
    match waitpid(Pid::from_raw(-1), None) {
        Ok(status) => println!("[Main] Child exited with status {:?}!", status),
        Err(e) => panic!("[Main] waitpid() failed with error code {}...", e),
    }
    println!("[Main] Won't be waiting for grandchild termination! Quitting!");
    sleep(Duration::from_millis(1000));
    println!("[Main] Bye!");
}

fn exec_or_die(s: &str)
{
    let name = CString::new(s).unwrap();
    match execv(&name, &[&name.clone()]) {
        Ok(_) => unreachable!("success"),
        Err(e) => unreachable!("execv() failed with error code {}...", e),
    }
}