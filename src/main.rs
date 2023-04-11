use tiny_http::{Header, Response, Server};

const PAGE_404: &str = include_str!("../site/404.html");

fn main() {
    let server = Server::http("0.0.0.0:3000").unwrap();

    for request in server.incoming_requests() {
        println!(
            "received request! method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );

        let response = Response::from_string(PAGE_404)
            .with_status_code(404)
            .with_header(
                Header::from_bytes("Content-Type", "text/html").unwrap(),
            );
        request.respond(response).unwrap();
    }
}
