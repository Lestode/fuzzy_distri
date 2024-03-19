use warp::Filter;

#[tokio::main]
async fn main() {
    // Define the ping route
    let ping = warp::path("ping").map(|| warp::reply::html("Pong!"));

    // Start the warp server for ping service
    warp::serve(ping).run(([0, 0, 0, 0], 8080)).await;
}
