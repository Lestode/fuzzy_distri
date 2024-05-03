use ctor::ctor;
use libc;
use shared_memory::ShmemConf;

#[ctor]
fn library_init() {
    let shmem = ShmemConf::new()
        .size(4096)
        .os_id("handler_addresses")
        .create()
        .expect("Couldn't create the shared memory segment");

    unsafe {
        // Get a mutable raw pointer to the shared memory
        let my_ptr = shmem.as_ptr() as *mut usize;
        *my_ptr = getpid as *const () as usize;
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
