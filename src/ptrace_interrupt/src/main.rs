use nix::{
    sys::{
        ptrace,
        wait::{waitpid, WaitStatus},
    },
    unistd::Pid,
};
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <pid to attach>", args[0]);
        process::exit(1);
    }

    let pid = Pid::from_raw(args[1].parse::<i32>().expect("Invalid PID"));
    ptrace::attach(pid).expect("Failed to attach to process");
    waitpid(pid, None).expect("Failed to wait for process stop");

    loop {
        ptrace::syscall(pid, None).expect("Failed to setup syscall trace");
        let status = waitpid(pid, None).expect("Failed to what for syscall");
        match status {
            WaitStatus::PtraceSyscall(_) => {
                let regs = ptrace::getregs(pid).expect("Failed to get registers");
                if regs.orig_rax == 39 {
                    // 39 is the syscall number for getpid on x86_64
                    println!("Intercepted getpid() call!");
                }
            }
            WaitStatus::Exited(_, code) => {
                println!("Process exited with code: {}", code);
                break;
            }
            _ => continue,
        }
    }

    ptrace::detach(pid).expect("Failed to detach");
}
