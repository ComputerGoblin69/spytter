use tiny_http::{Response, Server};

fn main() {
    let server = Server::http("0.0.0.0:3000").unwrap();

    for request in server.incoming_requests() {
        println!(
            "received request! method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );

        let response = Response::from_string(
            "Velkomm to Spytter and hello world!!!!!!!!!!!!",
        );
        request.respond(response).unwrap();
    }
}
