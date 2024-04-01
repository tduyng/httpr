use std::env;

use httpr::server;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let port = "4221";
    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();
    println!("Connection to port: {}", port);

    let directory = if let Some(dir) = args.get(2) {
        dir.clone()
    } else {
        eprintln!("No directory argument provided. Using default directory.");
        String::from("default")
    };

    server::run(listener, directory).await;
}
