use nix::{
    sys::{
        ptrace,
        wait::{waitpid, WaitStatus},
    },
    unistd::Pid,
};
use shared_memory::{Shmem, ShmemConf};
use std::{env, io::Read, thread};
use std::{fs::OpenOptions, mem, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <pid to attach>", args[0]);
        process::exit(1);
    }

    //let pid = Pid::from_raw(args[1].parse::<i32>().expect("Invalid PID"));
    //ptrace::attach(pid).expect("Failed to attach to process");
    //waitpid(pid, None).expect("Failed to wait for process stop");

    //loop {
    //ptrace::syscall(pid, None).expect("Failed to setup syscall trace");
    //let status = waitpid(pid, None).expect("Failed to what for syscall");
    //match status {
    //WaitStatus::PtraceSyscall(_) => {
    //println!("Syscall called");
    //let regs = ptrace::getregs(pid).expect("Failed to get registers");
    //if regs.orig_rax == 39 {
    //// 39 is the syscall number for getpid on x86_64
    //println!("Intercepted getpid() call!");
    //}
    //}
    //WaitStatus::Exited(_, code) => {
    //println!("Process exited with code: {}", code);
    //break;
    //}
    //_ => continue,
    //}
    //}

    //ptrace::detach(pid, None);

    /*Here we simply make sure that we're able to read from the shared memory */
    get_handler_pointer();
}

fn get_handler_pointer() -> usize {
    let mut mq = OpenOptions::new()
        .read(true)
        .create(true)
        .open("/handler_addres")
        .expect("Failed to open the message queue");
    // Buffer to hold the incoming bytes, the size of usize
    let mut buffer = [0u8; mem::size_of::<usize>()];

    // Receive the message
    mq.read(&mut buffer).expect("Failed to receive message");

    // Convert the received bytes back to usize
    let received_address = usize::from_ne_bytes(buffer);
    println!("Received address: {:x}", received_address);
    received_address
}
