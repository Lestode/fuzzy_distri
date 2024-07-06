use core::arch::asm;
use std::sync::atomic::{fence, Ordering};
use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    println!("main launched");
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(30) {
        let pid = unsafe { getpid_with_padding() };
        println!("Process ID obtained in main: {}", pid);
        thread::sleep(Duration::from_secs(1)); // Sleep for a bit to avoid spamming
    }
}

#[cfg(target_os = "linux")]
#[inline(never)]
unsafe fn getpid_with_padding() -> libc::pid_t {
    let pid: libc::pid_t;
    asm!(
        "mov rax, 39",  // syscall number for getpid on x86_64 Linux
        "syscall",      // make the syscall
        "mov {}, rax",  // store the result in the pid variable
        "nop", "nop", "nop", "nop", "nop",
        "nop", "nop", "nop", "nop", "nop",
        out(reg) pid,
        options(nostack, preserves_flags)
    );
    pid
}
