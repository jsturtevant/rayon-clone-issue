use nix::errno::Errno;
use nix::sched::{clone, CloneFlags};
use nix::sys::wait::{waitpid, WaitStatus};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::os::raw::c_int;
use std::process;
use nix::unistd::getpid;

use libc::SIGCHLD;
fn main() {
    let stack = &mut [0; 2024 * 1024];

    let mut v: Vec<i32> = Vec::new();
    for i in 1..=1000 {
        v.push(i);
    }

    let s1 = sum_of_squares(&v);
    println!("Sum of squares: {}", s1);

    let flags = unsafe { CloneFlags::from_bits_unchecked(SIGCHLD as c_int) };
    let pid = clone(
        Box::new(|| {
            println!("Hello from child process {}!", getpid());
            // uncomment these lines and it will hang
            // let s2 = sum_of_squares(&v);
            // println!("Sum of squares by child: {}", s2);
            println!("Exiting: {}", getpid());
            process::exit(0);
        }),
        stack,
        flags,
        None,
    )
    .unwrap();

    println!("I am the parent process with PID: {} with child process {} ", getpid(), pid); 
    let status = match waitpid(Some(pid),None) {
        Ok(WaitStatus::Signaled(_, sig, _)) => sig as i32,
        Ok(_) => 0,
        Err(Errno::ECHILD) => {
            println!("no process {}!", pid);
            0
        }
        Err(e) => {
            println!("error {}!",e);
            137
        }
    };

    println!("Child process exited: {}", status);

}

fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter() // <-- just change that!
         .map(|&i| i * i)
         .sum()
}
