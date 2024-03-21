use std::convert::Infallible;

use serde::{Deserialize, Serialize};
use warp::Reply;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum NetworkOperations {
    BlockData {},
    Delay { seconds_delay: u64 },
    RandomlyModify {},
    Pass {},
}

pub async fn syscall_sendto_handler() -> Result<impl Reply, Infallible> {
    println!("syscall sendto sent!");
    Ok(warp::reply::json(&NetworkOperations::Delay {
        seconds_delay: 10,
    }))
}
