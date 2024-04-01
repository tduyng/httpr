use std::{
    collections::HashMap,
    error::Error,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

struct HttpResponse {
    protocol: String,
    status_code: u32,
    status_text: String,
    body: Option<String>,
    headers: HashMap<String, String>,
}

impl HttpResponse {
    fn into_response_string(self) -> Result<String, Box<dyn Error>> {
        let mut response = format!(
            "{} {} {}\r\n",
            self.protocol, self.status_code, self.status_text
        );

        for (key, value) in self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value))
        }

        if let Some(body) = self.body {
            response.push_str("Content-Type: text/plain\r\n");
            response.push_str(&format!("Content-Length: {}\r\n\r\n", body.len()));
            response.push_str(&body);
        } else {
            response.push_str("\r\n");
        }

        Ok(response)
    }
}

fn not_found_error(protocol: &str) -> HttpResponse {
    HttpResponse {
        protocol: protocol.to_string(),
        status_code: 404,
        status_text: "Not Found".to_string(),
        body: None,
        headers: HashMap::new(),
    }
}

fn handle_request(mut stream: TcpStream) {
    let request = BufReader::new(&stream)
        .lines()
        .map(|result| result.expect("Failed to read request line"))
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let fist_line = request.first().unwrap().to_string();
    let mut line_parts = fist_line.splitn(3, ' '); // Split with limit 3
    let _method = line_parts.next().unwrap();
    let path = line_parts.next().unwrap();
    let protocol = line_parts.next().unwrap();

    let response = match path {
        path if path.starts_with("/echo/") => {
            let text = &path[6..];
            HttpResponse {
                protocol: protocol.to_string(),
                status_code: 200,
                status_text: "OK".to_string(),
                body: Some(text.to_string()),
                headers: HashMap::new(),
            }
        }
        "/" => HttpResponse {
            protocol: protocol.to_string(),
            status_code: 200,
            status_text: "OK".to_string(),
            body: None,
            headers: HashMap::new(),
        },
        _ => not_found_error(protocol),
    };

    let response_string = response.into_response_string().unwrap();

    stream
        .write_all(&response_string.into_bytes())
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
