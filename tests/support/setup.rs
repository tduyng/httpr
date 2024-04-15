use std::sync::Arc;

use http_server_starter_rust::server::Server;
use tokio::{net::TcpListener, sync::Mutex};

pub struct TestApp {
    pub server: Arc<Mutex<Server>>,
    pub address: String,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:4243").await.unwrap();
    let port = listener.local_addr().unwrap();
    let server = Arc::new(Mutex::new(Server::new()));
    let server_clone = server.clone();

    tokio::spawn(async move {
        let server = server_clone.lock().await;
        server
            .start_server(listener)
            .await
            .expect("Failed to start server");
    });

    TestApp {
        server: server.clone(),
        address: format!("http://127.0.0.1:{}", port),
    }
}
