use anyhow::Result;
use bytes::BytesMut;
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use crate::{Context, Request, Response, Router};

pub struct Server {
    router: Arc<Mutex<Router>>,
}

impl Default for Server {
    fn default() -> Self {
        Server::new()
    }
}

impl Server {
    pub fn new() -> Self {
        Server {
            router: Arc::new(Mutex::new(Router::new())),
        }
    }

    pub fn apply(&mut self, router: Router) {
        self.router = Arc::new(Mutex::new(router));
    }

    pub async fn listen(&mut self, address: SocketAddr) -> Result<()> {
        let listener = TcpListener::bind(address).await?;
        println!("Server started on {}", address);
        let router = self.router.clone();
        loop {
            let (stream, _addr) = listener.accept().await?;
            let router = router.clone();
            tokio::spawn(async move {
                if let Err(e) = Server::handle_connection(stream, router).await {
                    eprintln!("Error processing request: {}", e);
                }
            });
        }
    }

    async fn handle_connection(mut socket: TcpStream, router: Arc<Mutex<Router>>) -> Result<()> {
        let mut bytes = BytesMut::new();
        socket.read_buf(&mut bytes).await?;

        let buf = bytes.freeze();
        let request = Request::new(buf)?;
        let response = Response::default();
        let path = request.path.clone();
        let method = request.method;
        let mut ctx = Context::new(request, response);

        router.lock().await.route(&mut ctx, &method, &path).await;

        Ok(())
    }
}
