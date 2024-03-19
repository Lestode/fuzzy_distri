use std::{convert::Infallible, time::Duration};
use warp::http::Response;
use warp::Filter;

#[tokio::main]
async fn main() {
    let ping_url = "http://localhost:8080/ping";
    let pong = warp::path("pong").and_then(move || {
        let ping_url = ping_url.clone();
        async move {
            let client = reqwest::Client::new();
            loop {
                let res = client.get(ping_url).send().await;
                println!("Send a message to ping");
                match res {
                    Ok(resp) => {
                        let body = resp.text().await.unwrap_or_default();
                        println!("Ping Service responded with: {}", body);
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                    Err(_) => {
                        break Ok::<_, Infallible>(
                            Response::builder().body("Ping Service did not respond".to_string()),
                        );
                    }
                }
            }
        }
    });

    // Start the warp server for pong service
    warp::serve(pong).run(([0, 0, 0, 0], 8081)).await;
}
