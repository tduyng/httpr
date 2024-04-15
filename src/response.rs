use bytes::BytesMut;
use std::{
    fmt::{self, Write},
    sync::Arc,
};
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::Mutex};
use tracing::{error, info};

use crate::error::ServerError;

pub struct Response {
    status_code: u32,
    status_message: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}

impl Response {
    pub fn new() -> Self {
        Response {
            status_code: 200,
            status_message: "OK".to_string(),
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn status_code(mut self, code: u32, message: &str) -> Self {
        self.status_code = code;
        self.status_message = message.to_string();
        self
    }

    pub fn header(mut self, name: &str, val: &str) -> Self {
        self.headers.push((name.to_string(), val.to_string()));
        self
    }

    pub fn body(mut self, body: &[u8]) -> Self {
        self.body = body.to_vec();
        self
    }

    pub fn body_str(mut self, body: &str) -> Self {
        self.body = body.as_bytes().to_vec();
        self
    }

    pub async fn write_response(&self, stream: Arc<Mutex<TcpStream>>) -> Result<(), ServerError> {
        let mut buf = BytesMut::new();
        _ = write!(
            FastWrite(&mut buf),
            "HTTP/1.1 {} {}\r\n",
            self.status_code,
            self.status_message
        )
        .map_err(Ok::<fmt::Error, ServerError>);

        for (name, value) in &self.headers {
            _ = write!(FastWrite(&mut buf), "{}: {}\r\n", name, value)
                .map_err(Ok::<fmt::Error, ServerError>);
        }

        _ = write!(
            FastWrite(&mut buf),
            "Content-Length: {}\r\n",
            self.body.len()
        )
        .map_err(Ok::<fmt::Error, ServerError>);

        _ = write!(FastWrite(&mut buf), "\r\n").map_err(Ok::<fmt::Error, ServerError>);

        buf.extend_from_slice(&self.body);

        let mut stream = stream.lock().await;
        stream.write_all(&buf).await.map_err(|e| {
            error!("Failed to write response: {}", e);
            ServerError::IoError(e)
        })?;

        info!("Response sent successfully");
        Ok(())
    }
}

struct FastWrite<'a>(&'a mut BytesMut);

impl<'a> fmt::Write for FastWrite<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.extend_from_slice(s.as_bytes());
        Ok(())
    }
}
