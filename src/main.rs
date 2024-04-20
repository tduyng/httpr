use clap::Parser;
use http_server_starter_rust::{
    args::CliArgs,
    error::{internal_server_error, not_found, ServerError},
    request::{Request, RequestContext},
    routes::handle_routes,
};
use std::sync::Arc;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use tracing::{error, info, Level};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    let args = CliArgs::parse();
    let port = 4221;

    let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();
    info!("Server running on port {}", port);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("Server running on port {}", port);

                let stream = Arc::new(Mutex::new(stream));
                let args = args.clone();

                tokio::spawn(async move {
                    if let Ok(request) = Request::parse(stream.clone()).await {
                        let request_context = RequestContext::new(&request, &args);
                        handle_connection(stream.clone(), &request_context).await;
                    } else {
                        error!("Failed to read from socket");
                    }
                });
            }
            Err(e) => {
                error!("Error accepting connection: {}", e);
            }
        }
    }
}

async fn handle_connection(stream: Arc<Mutex<TcpStream>>, request_context: &RequestContext<'_>) {
    match handle_routes(request_context).await {
        Ok(response) => {
            if let Err(e) = response.write_response(stream).await {
                error!("Failed to write to socket: {}", e);
            }
        }
        Err(err) => {
            error!("Error handling request: {}", err);
            let response = match err {
                ServerError::NotFound => not_found(),
                _ => internal_server_error(),
            };
            if let Err(e) = response.write_response(stream.clone()).await {
                error!("Failed to write error response to socket: {}", e);
            }
        }
    };
}
