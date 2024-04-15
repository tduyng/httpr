use crate::{
    error::{bad_gateway, forbidden, internal_server_error, not_found, unauthorized, ServerError},
    request::{Request, RequestContext},
    routes::handle_routes,
    CliArgs,
};
use clap::Parser;
use std::sync::Arc;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use tracing::error;

pub struct Server;

impl Server {
    pub async fn start_server(listener: TcpListener) {
        let args = CliArgs::parse();

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let stream = Arc::new(Mutex::new(stream));
                    let args = args.clone();

                    tokio::spawn(async move {
                        if let Ok(request) = Request::parse(stream.clone()).await {
                            let request_context = RequestContext::new(&request, &args);
                            Server::handle_connection(stream.clone(), &request_context).await;
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

    async fn handle_connection(
        stream: Arc<Mutex<TcpStream>>,
        request_context: &RequestContext<'_>,
    ) {
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
                    ServerError::Unauthorized => unauthorized(),
                    ServerError::Forbidden => forbidden(),
                    ServerError::BadGateway => bad_gateway(),
                    _ => internal_server_error(),
                };
                if let Err(e) = response.write_response(stream.clone()).await {
                    error!("Failed to write error response to socket: {}", e);
                }
            }
        };
    }
}
