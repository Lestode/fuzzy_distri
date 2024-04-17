use tiny_http;

fn main() {
    env_logger::init();
    let server = tiny_http::Server::http("0.0.0.0:8080").unwrap();

    for request in server.incoming_requests() {
        let response = match request.url() {
            "/ping" => tiny_http::Response::from_string("Pong!").with_status_code(200),
            _ => tiny_http::Response::from_string("Not Found").with_status_code(404),
        };

        let _ = request.respond(response);
    }
}
