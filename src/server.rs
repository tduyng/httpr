use tokio::net::TcpListener;

use crate::routes;

pub async fn run(listener: TcpListener, directory: String) {
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("Retrieved a connection");
                let directory = directory.clone();
                tokio::spawn(async move {
                    routes::handle_request(stream, &directory).await;
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
