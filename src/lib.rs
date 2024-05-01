mod error;
mod http_request;
mod http_response;
pub mod tokens;
pub use error::*;
pub use http_request::*;
pub use http_response::*;
pub use httpstatus::{StatusClass, StatusCode};

use anyhow::Result;
use bytes::{Bytes, BytesMut};
use socket2::{Domain, Socket, Type};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    runtime,
};

#[derive(Default, Debug)]
pub struct HttpServer {}

impl HttpServer {
    pub fn new() -> Self {
        HttpServer {}
    }

    pub fn listen_blocking(&mut self, address: SocketAddr) -> Result<()> {
        let rt = runtime::Runtime::new()?;
        rt.block_on(self.listen(address))
    }

    pub async fn listen(&mut self, address: SocketAddr) -> Result<()> {
        let socket = Socket::new(Domain::IPV6, Type::STREAM, None)?;

        // Enable processing of both ipv6 and ipv4 packets
        socket.set_only_v6(false)?;

        // Stop processing requests right when a close/shutdown request is received
        socket.set_linger(Some(Duration::new(0, 0)))?;

        // Set our socket as non-blocking, which will result in
        // `read`, `write`, `recv` and `send` operations immediately
        // returning from their calls.
        // We want this to enable multiple threads to process sockets concurrently.
        socket.set_nonblocking(true)?;

        // Finally bind the socket to the correct interface/port and start to listen for new connection
        socket.bind(&address.into())?;
        socket.listen(128)?;
        let listener = TcpListener::from_std(socket.into())?;

        println!("started server on {}", address);
        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    tokio::spawn(async move {
                        HttpServer::process_request(socket, addr).await.unwrap_or_else(|e| {
                            println!("{}", e);
                        })
                    });
                }
                Err(e) => println!("couldn't get client: {:?}", e),
            }
        }
    }

    async fn process_request(mut socket: TcpStream, addr: SocketAddr) -> Result<()> {
        println!("received request from {}", addr);
        let mut bytes = BytesMut::new();
        let request_length = socket.read_buf(&mut bytes).await?;
        println!("got request:\n  length: {}", request_length);

        let mut request = http_request::Request::new();
        request.parse(Bytes::from(bytes))?;

        println!(
            "  method: {}\n  path: {}\n  version: HTTP/{}",
            request.method.unwrap(),
            request.path.unwrap(),
            request.version.unwrap()
        );
        println!("  headers:");
        for (header, value) in request.headers.iter() {
            println!(
                "    \"{}\":\"{}\"",
                header,
                String::from_utf8(value.to_vec()).expect("header to be string")
            );
        }

        let mut response = HttpResponse::default();
        response.set_header("x-powered-by", "rhttp");
        response.write(b"hello world");
        socket.write_all(&response.build()).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[tokio::test]
    async fn test_server_listen_and_respond() {
        let server_task = tokio::spawn(async {
            let mut server = HttpServer::new();
            server
                .listen(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 20241))
                .await
                .unwrap();
        });

        // Wait for the server to start listening
        tokio::time::sleep(Duration::from_secs(1)).await;

        let response = reqwest::get("http://localhost:20241").await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.text().await.unwrap(), "hello world");

        // Stop the server
        server_task.abort();
    }

    #[tokio::test]
    async fn test_server_listen_blocking_and_respond() {
        let mut server = HttpServer::new();
        server
            .listen_blocking(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 20242))
            .unwrap();

        let response = reqwest::get("http://localhost:20242").await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.text().await.unwrap(), "hello world");
    }
}
