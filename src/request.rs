use crate::{args::CliArgs, error::ServerError};
use bytes::BytesMut;
use std::{collections::HashMap, fmt, sync::Arc};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tracing::{error, info};

pub struct RequestContext<'a> {
    pub request: &'a Request,
    pub args: &'a CliArgs,
}

impl<'a> RequestContext<'a> {
    pub fn new(request: &'a Request, args: &'a CliArgs) -> Self {
        RequestContext { request, args }
    }
}

pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub async fn parse(stream: &Arc<Mutex<TcpStream>>) -> Result<Self, ServerError> {
        let mut buf = BytesMut::new();
        let mut stream = stream.lock().await;

        loop {
            let n = stream.read_buf(&mut buf).await?;
            if n == 0 {
                error!("Connection closed");
                return Err(ServerError::InternalError("Connection closed".to_string()));
            }

            if let Some(request) = parse_complete_request(&buf)? {
                info!(
                    "Request received: method={}, path={}",
                    request.method, request.path,
                );
                return Ok(request);
            }
        }
    }
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<HTTP Request {} {}", self.method, self.path)
    }
}

/// https://datatracker.ietf.org/doc/html/rfc2616#section-5.1
fn parse_complete_request(buf: &BytesMut) -> Result<Option<Request>, ServerError> {
    // Find the index of the first occurrence of double CRLF (\r\n\r\n) in the buffer
    let end_of_headers_index = find_end_of_headers(buf)?;

    if let Some(end_index) = end_of_headers_index {
        let headers_bytes = &buf[..end_index];
        let headers_string = String::from_utf8_lossy(headers_bytes);
        let headers = parse_headers(&headers_string);

        let body = parse_body(buf, end_index);

        let request_line = extract_request_line(buf, end_index);
        let request = request_line.map(|(method, path, version)| Request {
            method,
            path,
            version,
            headers,
            body,
        });

        Ok(request)
    } else {
        Ok(None)
    }
}

fn find_end_of_headers(buf: &BytesMut) -> Result<Option<usize>, ServerError> {
    // Find the index of the first occurrence of double CRLF (\r\n\r\n) in the buffer
    Ok(buf.windows(4).position(|w| w == b"\r\n\r\n"))
}

fn parse_headers(headers_string: &str) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    for line in headers_string.lines() {
        if let Some((lhs, rhs)) = line.split_once(": ") {
            headers.insert(lhs.trim().to_string(), rhs.trim().to_string());
        }
    }
    headers
}

fn parse_body(buf: &BytesMut, end_index: usize) -> Vec<u8> {
    let body = if let Some(content_length_str) = buf[end_index..].split(|&b| b == b'\n').next() {
        let content_length_str = String::from_utf8_lossy(content_length_str);
        content_length_str
            .trim()
            .parse::<usize>()
            .ok()
            .and_then(|content_length| {
                if buf.len() >= end_index + 4 + content_length {
                    Some(buf[end_index + 4..end_index + 4 + content_length].to_vec())
                } else {
                    None
                }
            })
    } else {
        None
    };

    body.unwrap_or_default()
}

fn extract_request_line(buf: &BytesMut, end_index: usize) -> Option<(String, String, String)> {
    buf[..end_index]
        .split(|&b| b == b'\n')
        .next()
        .and_then(|request_line_bytes| {
            let request_line = String::from_utf8_lossy(request_line_bytes)
                .trim()
                .to_string();
            parse_request_line(&request_line).ok()
        })
}

fn parse_request_line(request_line: &str) -> Result<(String, String, String), ServerError> {
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or_default().to_string();
    let path = parts.next().unwrap_or_default().to_string();
    let version = parts.next().unwrap_or_default().to_string();
    Ok((method, path, version))
}
