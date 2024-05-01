mod error;
mod http_request;
mod http_response;
pub mod tokens;
pub use error::*;
pub use http_request::*;
pub use http_response::*;
pub use httpstatus::{StatusClass, StatusCode};

use anyhow::Result;
use bytes::BytesMut;
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
        socket.read_buf(&mut bytes).await?;

        let mut res = HttpResponse::default();
        res.write(b"tee");
        socket.write_all(&res.build()).await?;

        Ok(())
    }
}
