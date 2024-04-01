use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream}, thread,
};

struct HttpRequest {
    path: String,
    headers: HashMap<String, String>,
}

impl HttpRequest {
    fn parse_request(lines: Vec<String>) -> Option<Self> {
        let first_line = lines.first()?;
        let mut parts = first_line.split_whitespace();
        let _method_str = parts.next()?;
        let path = parts.next()?.to_string();

        let mut headers = HashMap::new();
        for line in lines.iter().skip(1) {
            if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }
        }

        Some(Self { path, headers })
    }
}

struct HttpResponse {
    protocol: String,
    status_code: u32,
    status_text: String,
    body: Option<String>,
    headers: HashMap<String, String>,
}

impl HttpResponse {
    fn into_response_string(self) -> String {
        let mut response = format!(
            "{} {} {}\r\n",
            self.protocol, self.status_code, self.status_text
        );

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value))
        }

        if let Some(body) = &self.body {
            response.push_str("Content-Type: text/plain\r\n");
            response.push_str(&format!("Content-Length: {}\r\n\r\n", body.len()));
            response.push_str(body);
        } else {
            response.push_str("\r\n");
        }

        response
    }
}

fn handle_echo_request(path: &str) -> HttpResponse {
    let text = &path[6..];
    HttpResponse {
        protocol: "HTTP/1.1".to_string(),
        status_code: 200,
        status_text: "OK".to_string(),
        body: Some(text.to_string()),
        headers: HashMap::new(),
    }
}

fn handle_user_agent_request(headers: &HashMap<String, String>) -> HttpResponse {
    let user_agent = headers.get("User-Agent").cloned().unwrap_or_default();
    HttpResponse {
        protocol: "HTTP/1.1".to_string(),
        status_code: 200,
        status_text: "OK".to_string(),
        body: Some(user_agent),
        headers: HashMap::new(),
    }
}

fn handle_not_found_request() -> HttpResponse {
    HttpResponse {
        protocol: "HTTP/1.1".to_string(),
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

    if let Some(http_request) = HttpRequest::parse_request(request) {
        let response = match http_request.path.as_str() {
            path if path.starts_with("/echo/") => handle_echo_request(path),
            "/user-agent" => handle_user_agent_request(&http_request.headers),
            "/" => HttpResponse {
                protocol: "HTTP/1.1".to_string(),
                status_code: 200,
                status_text: "OK".to_string(),
                body: None,
                headers: HashMap::new(),
            },
            _ => handle_not_found_request(),
        };

        if let Err(e) = stream.write_all(&response.into_response_string().into_bytes()) {
            eprintln!("Failed to write HttpResponse: {}", e);
        }
    }
}

fn main() {
    let port = "4221";
    if let Ok(listener) = TcpListener::bind(format!("127.0.0.1:{}", port)) {
        println!("Connection to port: {}", port);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Retrieved a connection");
                    thread::spawn(move || {
                        handle_request(stream);
                    });
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    } else {
        eprintln!("Failed to make connection");
    }
}
