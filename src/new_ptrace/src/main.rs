use libc::{close, mmap, munmap, shm_open, MAP_SHARED, O_RDONLY, PROT_READ};
use nix::{
    sys::{
        ptrace,
        wait::{waitpid, WaitStatus},
    },
    unistd::Pid,
};
use std::ffi::CString;
use std::process;
use std::ptr;
use std::{env, thread};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <pid to attach>", args[0]);
        process::exit(1);
    }
    println!("ptrace got pid");
    let handler_adress = read_shared_memory("/mysharedmem").expect("Couldn't read shared memory");

    let pid = Pid::from_raw(args[1].parse::<i32>().expect("Invalid PID"));
    ptrace::attach(pid).expect("Failed to attach to process");
    waitpid(pid, None).expect("Failed to wait for process stop");

    loop {
        ptrace::syscall(pid, None).expect("Failed to setup syscall trace");
        let status = waitpid(pid, None).expect("Failed to what for syscall");
        match status {
            WaitStatus::PtraceSyscall(_) => {
                println!("Syscall called");
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

    ptrace::detach(pid, None).expect("Ptrace couldn't detach");
}

fn read_shared_memory(shm_name: &str) -> Result<usize, String> {
    unsafe {
        let name = CString::new(shm_name).map_err(|_| "Failed to create CString")?;
        let shm_fd = shm_open(name.as_ptr(), O_RDONLY, 0);
        if shm_fd == -1 {
            return Err("Error opening shared memory".to_string());
        }

        // Map the shared memory object
        let unconverted_address_pointer =
            mmap(ptr::null_mut(), 4096, PROT_READ, MAP_SHARED, shm_fd, 0);
        if unconverted_address_pointer == libc::MAP_FAILED {
            close(shm_fd);
            return Err("Error mapping shared memory".to_string());
        }

        // Read the data from the memory segment
        let adress_ptr = unconverted_address_pointer as *const usize;
        let passed_address = *adress_ptr;
        println!("passed address {}", passed_address);
        Ok(passed_address)
    }
}
