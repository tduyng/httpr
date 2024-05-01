use anyhow::Result;
use bytes::{Bytes, BytesMut};
use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    runtime,
};

use crate::{Request, Response};

#[derive(Default, Debug)]
pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }

    pub fn listen_blocking(&mut self, address: SocketAddr) -> Result<()> {
        let rt = runtime::Runtime::new()?;
        rt.block_on(self.listen(address))
    }

    async fn listen(&mut self, address: SocketAddr) -> Result<()> {
        let listener = TcpListener::bind(address).await?;
        println!("started server on {}", address);

        while let Ok((stream, _adrr)) = listener.accept().await {
            tokio::spawn(async move {
                if let Err(e) = Server::process_request(stream).await {
                    eprintln!("error processing request: {}", e);
                }
            });
        }
        Ok(())
    }

    async fn process_request(mut socket: TcpStream) -> Result<()> {
        let mut bytes = BytesMut::new();
        let request_length = socket.read_buf(&mut bytes).await?;

        let mut request = Request::new();
        request.parse(Bytes::from(bytes))?;

        Server::debug_request(request, request_length);

        let mut response = Response::default();
        response.set_header("x-powered-by", "rhttp");
        response.write(b"hello world");
        socket.write_all(&response.build()).await?;

        Ok(())
    }

    fn debug_request(request: Request, request_length: usize) {
        println!("got request:\n  length: {}", request_length);

        println!(
            "  method: {:?}\n  path: {}\n  version: HTTP/{}",
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

        if !request.body.is_empty() {
            println!(
                "  body: {}",
                String::from_utf8(request.body).unwrap_or("(not valid utf8)".to_string())
            );
        }
    }
}
