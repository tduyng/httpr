use http_server_starter_rust::{request::Request, response::Response, routes::handle_routes};
use std::sync::Arc;
use tokio::{net::TcpListener, sync::Mutex};

#[tokio::main]
async fn main() {
    let port = 4221;
    let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();
    println!("Server running on port {}", port);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("Received a connection");
                let stream = Arc::new(Mutex::new(stream));

                tokio::spawn(async move {
                    if let Ok(request) = Request::parse(stream.clone()).await {
                        let response = match handle_routes(&request.path).await {
                            Ok(response) => response,
                            Err(err) => {
                                eprintln!("Error handling request: {}", err);
                                Response::new()
                                    .body_str("Internal Server Error")
                                    .status_code(500, "Internal Server Error")
                                    .header("Content-Type", "text/plain")
                            }
                        };

                        if let Err(e) = response.write_response(stream.clone()).await {
                            eprintln!("Failed to write to socket: {}", e);
                        }
                    } else {
                        eprintln!("Failed to read from socket");
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
