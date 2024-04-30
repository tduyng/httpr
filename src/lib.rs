use anyhow::Result;
use socket2::{Domain, Socket, Type};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::time::Duration;

mod http_response;
pub use http_response::*;
pub use httpstatus::{StatusClass, StatusCode};

pub fn start_server() -> Result<()> {
    let socket = Socket::new(Domain::IPV6, Type::STREAM, None)?;
    let address: SocketAddr = "[::1]:2024".parse()?;
    socket.bind(&address.into())?;
    socket.set_linger(Some(Duration::new(3, 0)))?;
    socket.listen(128)?;

    let listener: TcpListener = socket.into();

    println!("starting server");
    loop {
        match listener.accept() {
            Ok((socket, addr)) => process_request(socket, addr)
                .map_err(|e| println!("Failed with: {}", e))
                .unwrap(),
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}

fn process_request(mut socket: TcpStream, addr: SocketAddr) -> Result<()> {
    println!("received request from {}", addr);

    let mut buffer = [0; 30000];
    socket.read(&mut buffer[..])?;

    let res = HttpResponse::default();
    std::io::stdout().write(&res.build())?;
    socket.write(&res.build())?;

    Ok(())
}
