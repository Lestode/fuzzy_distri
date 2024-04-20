use libc::{c_int, c_void, dlsym, sockaddr, socklen_t, ssize_t, RTLD_NEXT};
use std::ffi::CString;
use std::sync::Once;

pub unsafe fn call_original_accept4(
) -> Option<extern "C" fn(c_int, *mut sockaddr, *mut socklen_t, c_int) -> c_int> {
    let accept4_cstr = CString::new("accept4").expect("CString::new failed");
    let addr = dlsym(RTLD_NEXT, accept4_cstr.as_ptr());

    if addr.is_null() {
        None
    } else {
        Some(std::mem::transmute(addr))
    }
}

static ONCE: Once = Once::new();
static mut ORIGINAL_GETPID: Option<extern "C" fn() -> libc::pid_t> = None;

pub unsafe fn call_original_getpid() -> Option<extern "C" fn() -> libc::pid_t> {
    ONCE.call_once(|| {
        let getpid_cstr = CString::new("getpid").expect("CString::new failed");
        let addr = dlsym(RTLD_NEXT, getpid_cstr.as_ptr());
        ORIGINAL_GETPID = if addr.is_null() {
            None
        } else {
            Some(std::mem::transmute(addr))
        };
    });
    ORIGINAL_GETPID
}

pub unsafe fn call_original_sendto() -> Option<
    extern "C" fn(c_int, *const c_void, libc::size_t, c_int, *const sockaddr, socklen_t) -> ssize_t,
> {
    let sendto_cstr = CString::new("sendto").expect("CString::new failed");
    let addr = dlsym(RTLD_NEXT, sendto_cstr.as_ptr());

    if addr.is_null() {
        None
    } else {
        Some(std::mem::transmute(addr))
    }
}
