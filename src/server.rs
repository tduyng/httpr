use anyhow::Result;
use bytes::{Bytes, BytesMut};
use httpstatus::StatusCode;
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use crate::{find_request_path, Context, Request, Response, Route};

pub struct Server {
    routes: Arc<Mutex<Vec<Route>>>,
}

impl Default for Server {
    fn default() -> Self {
        Server::new()
    }
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
        let request_method = request.method;
        let response = Response::default();
        Self::debug_request(&mut request);

        let mut middlewares = Vec::new();
        for route in routes.lock().await.iter() {
            if let Some(method) = route.method {
                if Some(method) != request_method {
                    continue;
                }
            } else if request_method.is_some() {
                continue;
            }

            if let Ok(Some(request_path)) = find_request_path(&request, route) {
                middlewares.push((route.clone(), request_path.clone()));
            }
        }
        let mut ctx = Context::from(request, response);

        let mut err = false;
        for (route, path) in middlewares {
            {
                ctx.params = path.params;
            }

            let handler = route.handler;
            let fut = handler(&mut ctx);

            if let Err(e) = fut.await {
                err = true;
                println!("An error occurred on a middleware: {}", e);
                break;
            }

            if ctx.has_ended() {
                break;
            }
        }

        if err {
            ctx.response.clear();
            ctx.response.write_body(b"internal server error");
            ctx.response.status_code(StatusCode::InternalServerError);
        }

        if !ctx.is_raw() {
            socket.write_all(&ctx.response.build()).await?;
        }

        Ok(())
    }

    fn debug_request(request: &mut Request) {
        println!("Got request:");
        println!("  Method: {:?}", &request.method);
        println!("  Path: {}", &request.path.as_ref().unwrap());
        println!("  Version: HTTP/{}", &request.version.unwrap_or(0));
        println!("  Headers:");
        let headers = &mut request.headers.clone();
        for (header, value) in headers.iter() {
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
