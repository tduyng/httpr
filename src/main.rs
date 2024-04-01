use core::fmt;
use std::{
    io::{self, BufRead, Write},
    net::{TcpListener, TcpStream},
};

struct HttpResponse {
    protocol: String,
    status_code: u32,
    status_text: String,
}

impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}\r\n\r\n",
            self.protocol, self.status_code, self.status_text
        )
    }
}

fn handle_request(mut stream: TcpStream) {
    let reader = io::BufReader::new(&stream);
    let request: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let start_line = request.first().unwrap().to_string();
    let mut line_parts = start_line.split(' ');
    let method = line_parts.next().unwrap();
    let path = line_parts.next().unwrap();
    let protocol = line_parts.next().unwrap();

    let response = match path {
        "/" => HttpResponse {
            protocol: protocol.to_string(),
            status_code: 200,
            status_text: "OK".to_string(),
        },
        _ => HttpResponse {
            protocol: protocol.to_string(),
            status_code: 404,
            status_text: "Not Found".to_string(),
        },
    };

    let response_string = format!("{}", response);
    println!(
        "{} {} {} {}\r\n\r\n",
        method, protocol, response.status_code, response.status_text
    );

    stream
        .write_all(response_string.as_bytes())
        .expect("Failed to write HttpResponse");
}

fn main() {
    let port = "4221";
    let listener =
        TcpListener::bind(format!("127.0.0.1:{port}")).expect("Failed to make connection");
    println!("Connection to port: {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Retrieved a connection");

                handle_request(stream)
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
