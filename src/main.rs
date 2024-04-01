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

fn not_found_error(protocol: &str, headers: HashMap<String, String>) -> HttpResponse {
    HttpResponse {
        protocol: protocol.to_string(),
        status_code: 404,
        status_text: "Not Found".to_string(),
        body: None,
        headers,
    }
}

fn response_ok(
    protocol: &str,
    headers: HashMap<String, String>,
    body: Option<String>,
) -> HttpResponse {
    HttpResponse {
        protocol: protocol.to_string(),
        status_code: 200,
        status_text: "OK".to_string(),
        body,
        headers,
    }
}

fn parse_headers(request: Vec<String>) -> (HashMap<String, String>, Option<String>) {
    let mut headers = HashMap::new();

    let mut user_agent = None;
    for line in request.iter().skip(1) {  
        if line.is_empty() {
            continue;
        }

        let trimmed_line = line.trim();
        let mut parts = trimmed_line.splitn(2, ':');
        let key = parts.next().unwrap().to_string();
        let value = parts.next().unwrap_or("").trim().to_string();

        if key.to_lowercase() == "user-agent" {  
            user_agent = Some(value.clone());
        }
        headers.insert(key, value);
    }

    (headers, user_agent)
}

fn handle_request(mut stream: TcpStream) {
    let request = BufReader::new(&stream)
        .lines()
        .map(|result| result.expect("Failed to read request line"))
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let first_line = request.first().unwrap().to_string();
    let mut line_parts = first_line.splitn(3, ' '); // Split with limit 3
    let _method = line_parts.next().unwrap();
    let path = line_parts.next().unwrap();
    let protocol = line_parts.next().unwrap();
    let (headers, user_agent) = parse_headers(request);

    let response = match path {
        path if path.starts_with("/echo/") => {
            let text = &path[6..];
            response_ok(protocol, headers, Some(text.to_string()))
        }
        "/user-agent" => {
            let user_agent_value = user_agent.unwrap_or_default();  // Use default empty string if not found
            response_ok(protocol, headers.clone(), Some(user_agent_value))
        },
        "/" => response_ok(protocol, headers, None),
        _ => not_found_error(protocol, headers),
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
