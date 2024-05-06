use libc::getpid;
use shared_memory::{Shmem, ShmemConf};
use std::thread;
use std::time::Duration;

fn main() {
    let pid = unsafe { getpid() };
    println!("Process ID: {}", pid);
    let shmem = match ShmemConf::new().size(4096).flink("adress").open() {
        Ok(m) => {
            println!("Correctly opened in Main");
            return;
        }
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
}
