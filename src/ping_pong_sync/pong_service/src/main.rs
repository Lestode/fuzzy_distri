use tiny_http;
use reqwest;

fn main() {
    let server = tiny_http::Server::http("0.0.0.0:8081").unwrap();
    println!("Server running on http://0.0.0.0:8081");

    for request in server.incoming_requests() {
        if request.url() == "/pong" {
            let response = match send_ping_to("http://localhost:8080/ping") {
                Ok(_) => tiny_http::Response::from_string("all good"),
                Err(_) => tiny_http::Response::from_string("Ping Service did not respond").with_status_code(500),
            };
            let _ = request.respond(response);
        } else {
            let response = tiny_http::Response::from_string("Not Found").with_status_code(404);
            let _ = request.respond(response);
        }
    }
}

fn send_ping_to(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let res = client.get(url).send()?.text()?;
    println!("Ping Service responded with: {}", res);
    Ok(res)
}