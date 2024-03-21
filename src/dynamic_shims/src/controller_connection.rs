use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use tokio;

const CONTROLLER_URL: &str = "http://localhost:3030/sendto";

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum NetworkOperations {
    BlockData {},
    Delay { seconds_delay: u64 },
    RandomlyModify {},
    Pass {},
}

#[tokio::main]
pub async fn send_controller(syscall: &str) -> Result<NetworkOperations, Error> {
    let client = Client::new();
    let response = client.post(CONTROLLER_URL).send().await?;
    let network_op = response.json::<NetworkOperations>().await?;
    Ok(network_op)
}
