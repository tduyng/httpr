use std::{io::Write, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("Accepted new connection");
                println!("Starting server on port 4221");

                let response = "HTTP/1.1 200 OK\r\n\r\n";

                println!("Sending response: {}", response);
                _stream.write_all(response.as_bytes()).unwrap()
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
