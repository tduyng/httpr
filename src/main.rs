use clap::Parser;
use http_server_starter_rust::{
    error::{bad_gateway, forbidden, internal_server_error, not_found, unauthorized, ServerError},
    request::{Request, RequestContext},
    routes::handle_routes,
    CliArgs,
};
use std::sync::Arc;
use tokio::{net::TcpListener, sync::Mutex};

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    let port = 4221;
    let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();
    println!("Server running on port {}", port);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("Received a connection");
                let stream = Arc::new(Mutex::new(stream));
                let args = args.clone();

                tokio::spawn(async move {
                    if let Ok(request) = Request::parse(stream.clone()).await {
                        let request_context = RequestContext::new(&request, &args);

                        match handle_routes(&request_context).await {
                            Ok(response) => {
                                if let Err(e) = response.write_response(stream.clone()).await {
                                    eprintln!("Failed to write to socket: {}", e);
                                }
                            }
                            Err(err) => {
                                eprintln!("Error handling request: {}", err);
                                let response = match err {
                                    ServerError::NotFound => not_found(),
                                    ServerError::Unauthorized => unauthorized(),
                                    ServerError::Forbidden => forbidden(),
                                    ServerError::BadGateway => bad_gateway(),
                                    _ => internal_server_error(),
                                };
                                if let Err(e) = response.write_response(stream.clone()).await {
                                    eprintln!("Failed to write error response to socket: {}", e);
                                }
                            }
                        };
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
