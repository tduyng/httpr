use std::collections::HashMap;

use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
    net::TcpStream,
};

#[derive(PartialEq)]
pub enum RequestMethod {
    Get,
    Post,
}

pub struct HttpRequest {
    pub method: RequestMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    pub async fn parse_request(stream: &mut TcpStream) -> Option<Self> {
        let mut reader = BufReader::new(stream);
        let mut request_lines = Vec::new();
        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line).await.unwrap();
            if bytes_read == 0 {
                break;
            }
            if line.trim().is_empty() {
                break;
            }
            request_lines.push(line.trim().to_string());
        }

        let first_line = request_lines.first()?;
        let mut parts = first_line.split_whitespace();
        let method_str = parts.next()?.to_string();
        let method = match method_str.as_str() {
            "GET" => RequestMethod::Get,
            "POST" => RequestMethod::Post,
            _ => return None,
        };
        let path = parts.next()?.to_string();

        let mut headers = HashMap::new();
        // Read the request headers
        for line in &request_lines[1..] {
            if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }
        }

        // Read the request body
        let mut body = String::new();
        if method == RequestMethod::Post {
            if let Some(content_length) = headers
                .get("Content-Length")
                .and_then(|len| len.parse::<usize>().ok())
            {
                let mut buf = vec![0u8; content_length];
                reader.read_exact(&mut buf).await.unwrap();
                body = String::from_utf8(buf).unwrap();
            }
        }

        Some(Self {
            method,
            path,
            headers,
            body,
        })
    }
}
