use crate::Result;
use bytes::BytesMut;
use std::{fmt, io, sync::Arc};
use tokio::{io::AsyncReadExt, net::TcpStream, sync::Mutex};

pub struct Request {
    pub method: String,
    pub path: String,
    pub version: u8,
    pub headers: Vec<(String, Vec<u8>)>,
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
        }
    }

    pub async fn parse(stream: Arc<Mutex<TcpStream>>) -> Result<Self> {
        let mut buf = BytesMut::new();
        let mut stream = stream.lock().await;
        stream.read_buf(&mut buf).await?;
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut request = httparse::Request::new(&mut headers);
        let status = request
            .parse(&buf)
            .map_err(|e| {
                let msg = format!("failed to parse http request: {:?}", e);
                io::Error::new(io::ErrorKind::Other, msg)
            })
            .unwrap();

        match status {
            httparse::Status::Complete(amt) => {
                let method = request.method.unwrap().to_string();
                let path = request.path.unwrap().to_string();
                let version = request.version.unwrap();

                let parsed_headers: Vec<(String, Vec<u8>)> = request
                    .headers
                    .iter()
                    .map(|h| (h.name.to_string(), h.value.to_vec()))
                    .collect();

                let _ = buf.split_to(amt);

                Ok(Request {
                    method,
                    path,
                    version,
                    headers: parsed_headers,
                })
            }
            httparse::Status::Partial => Err("Partial request received".into()),
        }
    }
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<HTTP Request {} {}", self.method, self.path)
    }
}
