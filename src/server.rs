use anyhow::Result;
use bytes::{Bytes, BytesMut};
use httpstatus::StatusCode;
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    runtime,
    sync::Mutex,
};

use crate::{find_request_path, MiddlewareContext, Request, Response, Route};

#[derive(Default, Debug)]
pub struct Server {
    routes: Arc<Mutex<Vec<Route>>>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            routes: Arc::new(Mutex::new(vec![])),
        }
    }

    pub async fn add_route(&self, route: Route) {
        self.routes.lock().await.push(route);
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
            let routes = self.routes.clone();
            tokio::spawn(async move {
                if let Err(e) = Server::handle_connection(stream, routes).await {
                    eprintln!("Error processing request: {}", e);
                }
            });
        }
    }

    async fn handle_connection(mut socket: TcpStream, routes: Arc<Mutex<Vec<Route>>>) -> Result<()> {
        let mut bytes = BytesMut::new();
        socket.read_buf(&mut bytes).await?;

        let mut request = Request::new();
        request.parse(Bytes::from(bytes))?;

        let mut middlewares = Vec::new();
        for route in routes.lock().await.iter() {
            if let Some(method) = route.method {
                if Some(method) != request.method {
                    continue;
                }
            } else if request.method.is_some() {
                continue;
            }

            if let Ok(Some(request_path)) = find_request_path(&request, route) {
                middlewares.push((route.clone(), request_path.clone()));
            }
        }

        let mut response = Response::default();
        response.set_header("x-powered-by", "rhttp");
        Self::debug_request(&mut request);
        let ctx = Arc::new(Mutex::new(MiddlewareContext::new(request, response)));

        let mut err = false;
        for (route, path) in middlewares {
            {
                let mut x = ctx.lock().await;
                x.params = path.params.clone();
            }

            let handler = route.handler.clone();
            let fut = handler(ctx.clone());

            if let Err(e) = fut.await {
                err = true;
                println!("An error occurred on a middleware: {}", e);
                break;
            }

            if ctx.lock().await.has_ended() {
                break;
            }
        }

        if err {
            let mut ctx = ctx.lock().await;
            ctx.response.clear();
            ctx.response.write(b"internal server error");
            ctx.response.status_code(StatusCode::InternalServerError);
        }

        if !ctx.lock().await.is_raw() {
            socket.write_all(&ctx.lock().await.response.build()).await?;
        }

        Ok(())
    }

    fn debug_request(request: &mut Request) {
        println!("Got request:");
        println!("  Method: {:?}", &request.method);
        println!("  Path: {}", request.path.as_mut().unwrap());
        println!("  Version: HTTP/{}", &request.version.unwrap_or(0));
        println!("  Headers:");
        for (header, value) in request.headers.iter() {
            println!("    \"{}\": \"{}\"", header, std::str::from_utf8(value).unwrap_or(""));
        }
        if !&request.body.is_empty() {
            println!(
                "  Body: {}",
                std::str::from_utf8(&request.body).unwrap_or("(not valid utf-8)")
            );
        }
    }
}
