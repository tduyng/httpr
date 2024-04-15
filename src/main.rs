use http_server_starter_rust::server::Server;
use tokio::net::TcpListener;
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    let port = 4221;

    let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();
    let server = Server::new();
    _ = server.start_server(listener).await;
}
