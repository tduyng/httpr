use anyhow::Result;
use rhhtp::HttpServer;
use std::net::SocketAddr;
use tokio::runtime;

fn main() -> Result<()> {
    let server = HttpServer::new();
    let rt = runtime::Runtime::new()?;
    let address: SocketAddr = "[::1]:2024".parse()?;
    rt.block_on(server.listen(address))
}
