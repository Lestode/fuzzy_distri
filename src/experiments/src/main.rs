use libc;
use std::hint::black_box;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    sleep(Duration::new(10, 0));
    let start = Instant::now();
    for _ in 0..1000 {
        unsafe {
            black_box(libc::getpid());
        }
    }
    let total_duration = start.elapsed();
    let average_duration = total_duration / 100;

    println!(
        "Average duration of getpid() syscall: {:?}",
        average_duration
    );
}
