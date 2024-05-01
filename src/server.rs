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

    pub async fn listen(&mut self, address: SocketAddr) -> Result<()> {
        let listener = TcpListener::bind(address).await?;
        println!("Server started on {}", address);

        loop {
            let (stream, _addr) = listener.accept().await?;
            tokio::spawn(async move {
                if let Err(e) = Server::handle_connection(stream).await {
                    eprintln!("Error processing request: {}", e);
                }
            });
        }
    }

    async fn handle_connection(mut socket: TcpStream) -> Result<()> {
        let mut bytes = BytesMut::new();
        socket.read_buf(&mut bytes).await?;

        let mut request = Request::new();
        request.parse(Bytes::from(bytes))?;

        Server::debug_request(request);

        let mut response = Response::default();
        response.set_header("x-powered-by", "rhttp");
        response.write(b"hello world");

        socket.write_all(&response.build()).await?;
        Ok(())
    }

    fn debug_request(request: Request) {
        println!("Got request:");
        println!("  Method: {:?}", request.method);
        println!("  Path: {}", request.path.unwrap_or_default());
        println!("  Version: HTTP/{}", request.version.unwrap_or(0));
        println!("  Headers:");
        for (header, value) in request.headers.iter() {
            println!("    \"{}\": \"{}\"", header, std::str::from_utf8(value).unwrap_or(""));
        }
        if !request.body.is_empty() {
            println!(
                "  Body: {}",
                std::str::from_utf8(&request.body).unwrap_or("(not valid utf-8)")
            );
        }
    }
}
