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
    let pid_int = args[1].parse::<i32>().expect("Invalid PID");
    println!("passed PID {}", pid_int);

    let pid = Pid::from_raw(pid_int);
    ptrace::attach(pid).expect("Failed to attach to process");
    waitpid(pid, None).expect("Failed to wait for process stop");
    ptrace::setoptions(pid, ptrace::Options::PTRACE_O_TRACESYSGOOD)
        .expect("Failed to set ptrace options");

    let mut in_syscall = false;

    loop {
        ptrace::syscall(pid, None).expect("Failed to setup syscall trace");
        let status = waitpid(pid, None).expect("Failed to what for syscall");
        match status {
            WaitStatus::PtraceSyscall(_) => {
                let regs = ptrace::getregs(pid).expect("Failed to get registers");

                if regs.orig_rax == 39 {
                    // 39 is the syscall number for getpid on x86_64
                    println!("Intercepted getpid() call!");
                    // Memory address of the syscall instruction
                    let syscall_address = regs.rip - 2; // Adjust if necessary

                    // Read the 8 bytes starting at syscall_address
                    let original_instruction =
                        read_memory(pid, syscall_address).expect("Failed to read instruction");
                    let bytes: [u8; 8] = original_instruction.to_ne_bytes();

                    println!("Original instruction at 0x{:x}: {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
                                 syscall_address, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]);
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
fn read_memory(pid: Pid, addr: u64) -> Result<i64, nix::Error> {
    ptrace::read(pid, addr as *mut _)
}
unsafe fn modify_getpid_with_padding(pid: Pid, syscall_address: u64) -> Result<(), nix::Error> {
    // Read 16 bytes starting from the syscall instruction
    let mut instructions = [0u64; 2];
    instructions[0] = ptrace::read(pid, syscall_address as *mut _)?
        .try_into()
        .unwrap();
    instructions[1] = ptrace::read(pid, (syscall_address + 8) as *mut _)?
        .try_into()
        .unwrap();

    let bytes: [u8; 16] = unsafe { std::mem::transmute(instructions) };

    // Check if the instruction sequence matches our pattern
    if bytes[0] == 0x0f && bytes[1] == 0x05 && // syscall
       bytes[2] == 0x48 && bytes[3] == 0x89 && bytes[4] == 0xc0 && // mov rax, rax
       bytes[5] == 0x90 && bytes[6] == 0x90 && bytes[7] == 0x90
    // three NOPs
    {
        println!(
            "Found getpid with padding at address 0x{:x}",
            syscall_address
        );

        // Modify the instruction sequence
        let modified_instructions = [
            (instructions[0] & 0xFFFFFFFFFFu64) | (0x90909048u64 << 40),
            (instructions[1] & 0xFFFFFFFF00000000u64) | 0x90909090u64,
        ];

        // Write back the modified instructions
        ptrace::write(
            pid,
            syscall_address as *mut _,
            modified_instructions[0] as *mut _,
        )?;
        ptrace::write(
            pid,
            (syscall_address + 8) as *mut _,
            modified_instructions[1] as *mut _,
        )?;

        println!("Modified instructions at address 0x{:x}", syscall_address);
    }

    Ok(())
}
