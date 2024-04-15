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
    sync::{mpsc, Mutex},
};
use tracing::{error, info};

#[derive(Debug)]
pub struct Server {
    _shutdown: Mutex<Option<mpsc::Sender<()>>>,
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

impl Server {
    pub fn new() -> Self {
        Server {
            _shutdown: Mutex::new(None),
        }
    }

    pub async fn start_server(&self, listener: TcpListener) -> Result<(), ServerError> {
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);
        *self._shutdown.lock().await = Some(shutdown_tx);

        info!(
            "Server running on port {}",
            listener.local_addr().unwrap().port()
        );

        loop {
            tokio::select! {
                res = self.accept_connections(listener.accept().await) => {
                    if let Err(err) = res {
                        error!("Failed to accept connections: {}", err);
                    }
                }
                _ = shutdown_rx.recv() => {
                    info!("Received shutdown signal. Shutting down...");
                    return Ok(());
                }
            }
        }
    }

    async fn accept_connections(
        &self,
        result: Result<(TcpStream, std::net::SocketAddr), tokio::io::Error>,
    ) -> Result<(), ServerError> {
        let (stream, _) = result?;
        let stream = Arc::new(Mutex::new(stream));
        let args = CliArgs::parse();

        tokio::spawn(async move {
            if let Ok(request) = Request::parse(stream.clone()).await {
                let request_context = RequestContext::new(&request, &args);
                if let Err(e) = handle_routes(&request_context).await {
                    error!("Error handling request: {}", e);
                    let response = match e {
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
            } else {
                error!("Failed to read from socket");
            }
        });

        Ok(())
    }

    pub async fn shutdown(&self) {
        if let Some(shutdown_tx) = &*self._shutdown.lock().await {
            let _ = shutdown_tx.send(()).await;
        }
    }
}
