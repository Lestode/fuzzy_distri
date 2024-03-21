mod controller_connection;
use libc::{sockaddr, socklen_t, ssize_t};
use std::{
    os::raw::{c_int, c_void},
    thread,
    time::Duration,
};

use crate::controller_connection::NetworkOperations;

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
    match controller_connection::send_controller("sendto") {
        Err(_) => {
            //TODO
            println!("For now, do nothing");
            0
        }
        Ok(net_op) => match net_op {
            NetworkOperations::BlockData {} => {
                println!("Data blocked");
                0
            }
            NetworkOperations::Pass {} => unsafe {
                libc::sendto(sockfd, buf, len, flags, dest_addr, addrlen)
            },
            NetworkOperations::Delay { seconds_delay } => {
                thread::sleep(Duration::from_secs(seconds_delay));
                unsafe { libc::sendto(sockfd, buf, len, flags, dest_addr, addrlen) }
            }
            NetworkOperations::RandomlyModify {} => 0, //TODO
        },
    }
}
