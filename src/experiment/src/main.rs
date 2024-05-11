use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    println!("main launched");
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(30) {
        let pid = unsafe { libc::getpid() };
        println!("Process ID obtained in main: {}", pid);
        thread::sleep(Duration::from_secs(1)); // Sleep for a bit to avoid spamming
    }
}
