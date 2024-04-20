use crate::{error::ServerError, CliArgs};
use bytes::BytesMut;
use std::{fmt, sync::Arc};
use tokio::{io::AsyncReadExt, net::TcpStream, sync::Mutex};
use tracing::{error, info};

pub struct Request {
    pub method: String,
    pub path: String,
    pub version: u8,
    pub headers: Vec<(String, Vec<u8>)>,
    pub body: Vec<u8>,
}

impl Default for Request {
    fn default() -> Self {
        Self::new()
    }
}

impl Request {
    pub fn new() -> Self {
        Request {
            method: String::new(),
            path: String::new(),
            version: 1,
            headers: Vec::new(),
            body: Vec::new(),
        }
    }
    pub async fn parse(stream: Arc<Mutex<TcpStream>>) -> Result<Self, ServerError> {
        let mut buf = BytesMut::new();
        let mut stream = stream.lock().await;

        loop {
            let n = stream.read_buf(&mut buf).await?;
            if n == 0 {
                error!("Connection closed");
                return Err(ServerError::InternalError("Connection closed".to_string()));
            }

            if let Some(request) = Self::parse_complete_request(&mut buf)? {
                info!(
                    "Request received: method={}, path={}, headers={:?}, body={}",
                    request.method,
                    request.path,
                    format_headers(&request.headers),
                    String::from_utf8_lossy(&request.body)
                );
                return Ok(request);
            }
        }
    }

    fn parse_complete_request(buf: &mut BytesMut) -> Result<Option<Self>, ServerError> {
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut request = httparse::Request::new(&mut headers);
        let status = request.parse(buf);

        match status {
            Ok(httparse::Status::Complete(_amt)) => {
                let method = request.method.unwrap().to_string();
                let path = request.path.unwrap().to_string();
                let version = request.version.unwrap();
                let parsed_headers: Vec<(String, Vec<u8>)> = request
                    .headers
                    .iter()
                    .map(|h| (h.name.to_string(), h.value.to_vec()))
                    .collect();

                let content_length: Option<usize> =
                    parsed_headers.iter().find_map(|(name, value)| {
                        if name.to_lowercase() == "content-length" {
                            Some(std::str::from_utf8(value).ok()?.parse().ok()?)
                        } else {
                            None
                        }
                    });

                let content_length = content_length.unwrap_or(0);
                // I use the \r\n\r\n sequence to identify the end of the headers and the start of the body
                let headers_end_index = buf
                    .windows(4)
                    .position(|window| window == b"\r\n\r\n")
                    .unwrap_or(0);

                if headers_end_index > 0 && buf.len() >= headers_end_index + 4 + content_length {
                    let body_bytes = buf.split_to(headers_end_index + 4 + content_length);
                    let body_slice = &body_bytes[headers_end_index + 4..];
                    let body_vec = body_slice.to_vec();

                    return Ok(Some(Request {
                        method,
                        path,
                        version,
                        headers: parsed_headers,
                        body: body_vec,
                    }));
                }
            }
            Ok(httparse::Status::Partial) => {}
            Err(_) => return Err(ServerError::ParseError("Error parsing request".into())),
        }

        Ok(None)
    }
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<HTTP Request {} {}", self.method, self.path)
    }
}

pub struct RequestContext<'a> {
    pub request: &'a Request,
    pub args: &'a CliArgs,
}

impl<'a> RequestContext<'a> {
    pub fn new(request: &'a Request, args: &'a CliArgs) -> Self {
        RequestContext { request, args }
    }
}

fn format_headers(headers: &[(String, Vec<u8>)]) -> String {
    let mut formatted_headers = String::new();
    for (name, value) in headers {
        if let Ok(value_str) = std::str::from_utf8(value) {
            formatted_headers.push_str(&format!("{}: {}, ", name, value_str));
        } else {
            formatted_headers.push_str(&format!("{}: {:?}, ", name, value));
        }
    }
    // Remove the trailing comma and space if there are headers
    if !formatted_headers.is_empty() {
        formatted_headers.pop();
        formatted_headers.pop();
    }
    formatted_headers
}
