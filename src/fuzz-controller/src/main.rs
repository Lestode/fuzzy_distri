mod syscall_handlers;
use warp::Filter;

#[tokio::main]
async fn main() {
    let syscall_sendto_route = warp::get()
        .and(warp::path("sendto"))
        .and_then(syscall_handlers::syscall_sendto_handler);

    warp::serve(syscall_sendto_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
