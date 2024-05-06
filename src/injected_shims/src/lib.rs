use ctor::ctor;
use libc;
use shared_memory::ShmemConf;
use std::process::Command;

#[ctor]
fn library_init() {
    eprintln!("library init called");
    let shmem = match ShmemConf::new().size(4096).flink("adress").create() {
        Ok(m) => {
            println!("Correctly created");
            m
        }
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    eprintln!("segment created");
    unsafe {
        let my_ptr = shmem.as_ptr() as *mut usize;
        *my_ptr = getpid as *const () as usize;
    }

    eprintln!("{}", getpid as *const () as usize);
    let output = Command::new("whoami")
        .output() // Execute the command and capture the output
        .expect("Failed to execute command");

    // Check if the command was executed successfully
    if output.status.success() {
        // Convert the output bytes to a String
        let output_string = String::from_utf8_lossy(&output.stdout);

        // Print the output
        println!("Output of `ipcs -m`:\n{}", output_string);
    } else {
        // If the command failed, handle errors (e.g., command not found)
        let error_string = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", error_string);
    }
}

#[no_mangle]
pub extern "C" fn getpid() -> libc::pid_t {
    //unsafe {
    //let original_getpid =
    //dlsym::call_original_getpid().expect("Failed to find the original accept4");
    //original_getpid()
    //}
    0
}
