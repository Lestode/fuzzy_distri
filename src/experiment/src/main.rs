use libc::getpid;
use shared_memory::{Shmem, ShmemConf};
use std::thread;
use std::time::Duration;

fn main() {
    let pid = unsafe { getpid() };
    println!("Process ID: {}", pid);
}
