use libc::{sockaddr, socklen_t, ssize_t};
use std::os::raw::{c_int, c_void};

#[no_mangle]
pub extern "C" fn sendto(
    sockfd: c_int,
    buf: *const c_void,
    len: libc::size_t,
    flags: c_int,
    dest_addr: *const sockaddr,
    addrlen: socklen_t,
) -> ssize_t {
    println!("Shim `sendto` called!");
    unsafe { libc::sendto(sockfd, buf, len, flags, dest_addr, addrlen) }
}
