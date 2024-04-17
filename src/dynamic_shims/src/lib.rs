mod controller_connection;
mod dlsym;
use libc::{c_int, c_void, sockaddr, socklen_t, ssize_t};

use std::{thread, time::Duration};

use crate::controller_connection::NetworkOperations;

//#[no_mangle]
//pub extern "C" fn sendto(
//sockfd: c_int,
//buf: *const c_void,
//len: libc::size_t,
//flags: c_int,
//dest_addr: *const sockaddr,
//addrlen: socklen_t,
//) -> ssize_t {
//println!("Shim `sendto` called!");
//match controller_connection::send_controller("sendto") {
//Err(_) => {
////TODO
//println!("For now, do nothing");
//0
//}
//Ok(net_op) => match net_op {
//NetworkOperations::BlockData {} => {
//println!("Data blocked");
//0
//}
//NetworkOperations::Pass {} => unsafe {
//println!("Data passed");
//libc::sendto(sockfd, buf, len, flags, dest_addr, addrlen)
//},
//NetworkOperations::Delay { seconds_delay } => {
//println!("Data delayed");
//thread::sleep(Duration::from_secs(seconds_delay));
//unsafe { libc::sendto(sockfd, buf, len, flags, dest_addr, addrlen) }
//}
//NetworkOperations::RandomlyModify {} => 0, //TODO
//},
//}
//}
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
    unsafe {
        let original_sendto =
            dlsym::call_original_sendto().expect("Failed to find the original sendto");
        original_sendto(sockfd, buf, len, flags, dest_addr, addrlen)
    }
}

//#[no_mangle]
//pub extern "C" fn recvfrom(
//sockfd: c_int,
//buf: *mut c_void,
//len: libc::size_t,
//flags: c_int,
//src_addr: *mut sockaddr,
//addrlen: *mut socklen_t,
//) -> ssize_t {
//println!("Shim `recvfrom` called!");
//unsafe { libc::recvfrom(sockfd, buf, len, flags, src_addr, addrlen) }
//}

#[no_mangle]
pub extern "C" fn accept4(
    sockfd: c_int,
    addr: *mut sockaddr,
    addrlen: *mut socklen_t,
    flags: c_int,
) -> c_int {
    println!("Shim `accept4` called!");
    unsafe {
        let original_accept4 =
            dlsym::call_original_accept4().expect("Failed to find the original accept4");
        original_accept4(sockfd, addr, addrlen, flags)
    }
}

#[no_mangle]
pub extern "C" fn getpid() -> libc::pid_t {
    println!("Shim `getpid` called!");
    unsafe {
        let original_getpid =
            dlsym::call_original_getpid().expect("Failed to find the original accept4");
        original_getpid()
    }
}
