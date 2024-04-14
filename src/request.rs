use crate::{CliArgs, Result};
use bytes::BytesMut;
use std::{fmt, sync::Arc};
use tokio::{io::AsyncReadExt, net::TcpStream, sync::Mutex};

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

    pub async fn parse(stream: Arc<Mutex<TcpStream>>) -> Result<Self> {
        let mut buf = BytesMut::new();
        let mut stream = stream.lock().await;

        // Read from the stream until a complete HTTP request is received
        loop {
            let n = stream.read_buf(&mut buf).await?;
            if n == 0 {
                return Err("Connection closed".into());
            }

            let mut headers = [httparse::EMPTY_HEADER; 16];
            let mut request = httparse::Request::new(&mut headers);
            let status = request.parse(&buf)?;

            match status {
                httparse::Status::Complete(amt) => {
                    let parsed_bytes = amt;
                    let method = request.method.unwrap().to_string();
                    let path = request.path.unwrap().to_string();
                    let version = request.version.unwrap();
                    let parsed_headers: Vec<(String, Vec<u8>)> = request
                        .headers
                        .iter()
                        .map(|h| (h.name.to_string(), h.value.to_vec()))
                        .collect();
                    let body_start = buf.len() - parsed_bytes;
                    let body = buf.split_to(body_start).to_vec();

                    return Ok(Request {
                        method,
                        path,
                        version,
                        headers: parsed_headers,
                        body,
                    });
                }
                httparse::Status::Partial => {}
            }
        }
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
