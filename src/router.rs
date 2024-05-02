use anyhow::Result;
use std::{collections::BTreeMap, fmt::Debug, future::Future, pin::Pin, sync::Arc};
use tokio::net::TcpStream;

use crate::{Method, Request, Response, Server};

pub trait HandlerFn: Fn(&mut Context) -> Handler + Send + Sync + 'static {}
pub type Handler = Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'static>>;

pub struct Context {
    pub request: Request,
    pub response: Response,
    pub params: BTreeMap<String, RequestPathParams>,
    pub socket: Option<TcpStream>,
    ended: bool,
    raw: bool,
}

impl Default for Context {
    fn default() -> Self {
        Context::new()
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            socket: None,
            request: Request::new(),
            response: Response::new(),
            ended: false,
            params: BTreeMap::new(),
            raw: false,
        }
    }

    pub fn from(request: Request, response: Response) -> Self {
        Self {
            socket: None,
            request,
            response,
            ended: false,
            params: BTreeMap::new(),
            raw: false,
        }
    }

    pub fn set_raw(&mut self, val: bool) {
        self.raw = val;
    }

    pub fn is_raw(&mut self) -> bool {
        self.raw
    }

    pub fn end(&mut self) {
        self.ended = true
    }

    pub fn has_ended(&self) -> bool {
        self.ended
    }
}

pub struct Route {
    pub path: String,
    pub method: Option<Method>,
    pub handler: Arc<Box<dyn HandlerFn>>,
}

impl Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("path", &self.path)
            .field("method", &self.method)
            .field("handler", &"[handlerFn]".to_string())
            .finish()
    }
}

impl Clone for Route {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            method: self.method,
            handler: self.handler.clone(),
        }
    }
}

pub trait Router<F>: Sync
where
    F: HandlerFn,
{
    fn handle(&mut self, method: Method, path: &str, handler: F) -> impl Future<Output = &mut Self> + Send;
    fn any(&mut self, path: &str, handler: F) -> impl Future<Output = &mut Self> + Send;
    fn get(&mut self, path: &str, handler: F) -> impl Future<Output = &mut Self> + Send;
    fn head(&mut self, path: &str, handler: F) -> impl Future<Output = &mut Self> + Send;
    fn post(&mut self, path: &str, handler: F) -> impl Future<Output = &mut Self> + Send;
    fn put(&mut self, path: &str, handler: F) -> impl Future<Output = &mut Self> + Send;
    fn delete(&mut self, path: &str, handler: F) -> impl Future<Output = &mut Self> + Send;
    fn connect(&mut self, path: &str, handler: F) -> impl Future<Output = &mut Self> + Send;
    fn options(&mut self, path: &str, handler: F) -> impl Future<Output = &mut Self> + Send;
    fn trace(&mut self, path: &str, handler: F) -> impl Future<Output = &mut Self> + Send;
}

#[derive(Debug, Clone)]
pub struct RequestPath {
    pub path: String,
    pub params: BTreeMap<String, RequestPathParams>,
}

#[derive(Debug, Clone)]
pub struct RequestPathParams {
    pub param: String,
    pub value: String,
}

pub fn find_request_path(request: &Request, route: &Route) -> Result<Option<RequestPath>> {
    let request_path = request.path.clone().unwrap_or_default();
    let request_segments = request_path.split('/').peekable();

    let route_path = &route.path;
    let mut route_segments = route_path.split('/').peekable();

    let mut params: BTreeMap<String, RequestPathParams> = BTreeMap::new();

    for request_segment in request_segments {
        let route_segment = match route_segments.next() {
            Some(val) => val,
            None => return Ok(None),
        };

        match route_segment {
            "*" => {
                if route_segments.peek().is_none() && !route_path.ends_with('/') {
                    break;
                }

                params.insert(
                    "*".to_string(),
                    RequestPathParams {
                        param: "*".to_string(),
                        value: request_segment.to_string(),
                    },
                );
            }

            s if s.starts_with(':') => {
                params.insert(
                    s.to_string(),
                    RequestPathParams {
                        param: s.to_string(),
                        value: request_segment.to_string(),
                    },
                );
            }

            s if s != request_segment => return Ok(None),

            _ => {}
        }
    }

    let path = RequestPath {
        path: request_path,
        params,
    };

    Ok(Some(path))
}

impl<F> Router<F> for Server
where
    F: HandlerFn,
{
    async fn handle(&mut self, method: Method, path: &str, handler: F) -> &mut Self {
        let handler: Arc<Box<dyn HandlerFn>> = Arc::new(Box::new(handler));
        let route: Route = Route {
            path: path.to_string(),
            method: Some(method),
            handler,
        };

        self.add_route(route).await;
        self
    }

    async fn any(&mut self, path: &str, handler: F) -> &mut Self {
        let handler: Arc<Box<dyn HandlerFn>> = Arc::new(Box::new(handler));
        let route: Route = Route {
            path: path.to_string(),
            method: None,
            handler,
        };
        self.add_route(route).await;
        self
    }

    async fn get(&mut self, path: &str, handler: F) -> &mut Self {
        self.handle(Method::GET, path, handler).await
    }
    async fn head(&mut self, path: &str, handler: F) -> &mut Self {
        self.handle(Method::HEAD, path, handler).await
    }
    async fn post(&mut self, path: &str, handler: F) -> &mut Self {
        self.handle(Method::POST, path, handler).await
    }
    async fn put(&mut self, path: &str, handler: F) -> &mut Self {
        self.handle(Method::PUT, path, handler).await
    }
    async fn delete(&mut self, path: &str, handler: F) -> &mut Self {
        self.handle(Method::DELETE, path, handler).await
    }
    async fn connect(&mut self, path: &str, handler: F) -> &mut Self {
        self.handle(Method::CONNECT, path, handler).await
    }
    async fn options(&mut self, path: &str, handler: F) -> &mut Self {
        self.handle(Method::OPTIONS, path, handler).await
    }
    async fn trace(&mut self, path: &str, handler: F) -> &mut Self {
        self.handle(Method::TRACE, path, handler).await
    }
}
