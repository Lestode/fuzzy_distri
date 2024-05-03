use nix::{
    sys::{
        ptrace,
        wait::{waitpid, WaitStatus},
    },
    unistd::Pid,
};
use shared_memory::{Shmem, ShmemConf};
use std::env;
use std::process;

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
    unsafe {
        get_handler_pointer();
    }
}

unsafe fn get_handler_pointer() -> usize {
    let shmem = ShmemConf::new()
        .size(4096)
        .os_id("handler_addresses")
        .open()
        .expect("Couldn't open the shared memory segment");

    unsafe {
        // Get a mutable raw pointer to the shared memory
        let my_ptr = shmem.as_ptr() as *const usize;
        let original_function_pointer = *my_ptr as *const () as usize;
        println!("function pointer: {}", original_function_pointer);
        return original_function_pointer;
    }
}
