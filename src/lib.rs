mod http_response;
use bytes::BytesMut;
pub use http_response::*;
pub use httpstatus::{StatusClass, StatusCode};

use anyhow::Result;
use socket2::{Domain, Socket, Type};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::time::Duration;

#[derive(Default, Debug)]
pub struct HttpServer {}

impl HttpServer {
    pub fn new() -> Self {
        HttpServer {}
    }

    pub fn listen(self, address: SocketAddr) -> Result<()> {
        let socket = Socket::new(Domain::IPV6, Type::STREAM, None)?;
        socket.bind(&address.into())?;
        socket.set_linger(Some(Duration::new(0, 0)))?;
        socket.listen(128)?;

        let listener: TcpListener = socket.into();

        println!("starting server on {}", address);
        loop {
            match listener.accept() {
                Ok((socket, addr)) => process_request(socket, addr)
                    .map_err(|e| println!("Failed with: {}", e))
                    .unwrap(),
                Err(e) => println!("couldn't get client: {:?}", e),
            }
        }
    }
}

fn process_request(mut socket: TcpStream, addr: SocketAddr) -> Result<()> {
    println!("received request from {}", addr);
    let mut bytes = BytesMut::new();
    socket.read_exact(&mut bytes[..])?;

    let mut res = HttpResponse::default();
    res.write(b"tee");
    socket.write_all(&res.build())?;

    Ok(())
}
