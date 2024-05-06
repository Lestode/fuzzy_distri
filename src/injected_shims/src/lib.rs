use ctor::ctor;
use libc;
use posix_mq::{OpenOptions, WriteMsg};
use std::{fs::OpenOptions, process::Command};

#[ctor]
fn library_init() {
    eprintln!("library init called");
    let mq = OpenOptions::new()
        .write(true)
        .create(true)
        .open("/handler_adress")
        .expect("Failed to open the message queue");

    let adress = getpid as *const () as usize;
    let bytes = adress.to_ne_bytes();

    mq.send(&bytes, 1).expect("Failed to send message");
    println!("Message sent");
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
