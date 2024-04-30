use anyhow::Result;
use rhhtp::HttpServer;
use std::net::SocketAddr;

fn main() -> Result<()> {
    let mut server = HttpServer::new();
    let address: SocketAddr = "[::1]:2024".parse()?;
    server.listen_blocking(address)
}
