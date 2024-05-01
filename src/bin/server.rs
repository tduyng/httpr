use anyhow::Result;
use rhhtp::Server;
use std::net::SocketAddr;

fn main() -> Result<()> {
    let mut server = Server::new();
    let address: SocketAddr = "[::1]:2024".parse()?;
    server.listen_blocking(address)
}
