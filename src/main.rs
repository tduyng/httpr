use http_server_starter_rust::server::Server;
use tokio::net::TcpListener;
use tracing::{info, Level};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    let port = 4221;

    let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();
    info!("Server running on port {}", port);
    let server = Server::new();
    _ = server.start_server(listener).await;
}
