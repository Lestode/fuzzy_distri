use libc::getpid;
use std::thread;
use std::time::Duration;

fn main() {
    let pid = unsafe { getpid() };
    println!("Process ID: {}", pid);
    thread::sleep(Duration::from_secs(60));
}
