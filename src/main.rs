use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let port = 4221;
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port))
        .await
        .unwrap();
    println!("Server running on port {}", port);

    loop {
        match listener.accept().await {
            Ok((mut stream, _)) => {
                println!("Received a connection");
                tokio::spawn(async move {
                    if let Err(e) = stream
                        .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello World")
                        .await
                    {
                        eprintln!("Failed to write to socket: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
