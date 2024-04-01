use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("Accepted new connection");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
