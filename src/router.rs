use crate::{Context, Method};
use std::{collections::HashMap, future::Future, pin::Pin};

pub type HandlerFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
pub type Handler = Box<dyn Fn(&mut Context) -> HandlerFuture + Send + Sync + 'static>;

pub struct Router {
    routes: HashMap<String, Handler>,
}

impl Default for Router {
    fn default() -> Self {
        Router::new()
    }
}

impl Router {
    pub fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        self.add_route(Method::GET, path, handler);
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        self.add_route(Method::POST, path, handler);
    }

    pub fn put(&mut self, path: &str, handler: Handler) {
        self.add_route(Method::PUT, path, handler);
    }

    pub fn delete(&mut self, path: &str, handler: Handler) {
        self.add_route(Method::DELETE, path, handler);
    }

    pub fn trace(&mut self, path: &str, handler: Handler) {
        self.add_route(Method::TRACE, path, handler);
    }

    pub fn connect(&mut self, path: &str, handler: Handler) {
        self.add_route(Method::CONNECT, path, handler);
    }

    pub fn options(&mut self, path: &str, handler: Handler) {
        self.add_route(Method::OPTIONS, path, handler);
    }

    pub fn any(&mut self, path: &str, handler: Handler) {
        self.add_route(Method::ANY, path, handler);
    }

    pub fn add_route(&mut self, method: Method, path: &str, handler: Handler) {
        let key = format!("{} {}", method, path);
        self.routes.insert(key, handler);
    }

    pub async fn route(&self, ctx: &mut Context, method: &Method, path: &str) {
        let (handler, params) = self.match_route(method, path);
        if let Some(handler) = handler {
            ctx.set_path(path.to_string());
            ctx.set_path_params(params);

            handler(ctx).await;
        } else {
            let mut response = ctx.response.lock().await;
            response.status_code(httpstatus::StatusCode::NotFound);
            response.write_body(b"Not Found");
        }
    }

    fn match_route(&self, method: &Method, path: &str) -> (Option<&Handler>, HashMap<String, String>) {
        for (route, handler) in &self.routes {
            let parts: Vec<&str> = route.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let route_method = Method::try_from(parts[0]).unwrap_or(Method::GET);
                if &route_method == method || route_method == Method::ANY {
                    if let Some(params) = self.match_path(parts[1], path) {
                        return (Some(handler), params);
                    }
                }
            }
        }
        (None, HashMap::new())
    }

    fn match_path(&self, route: &str, path: &str) -> Option<HashMap<String, String>> {
        let mut route_parts = route.split('/');
        let mut path_parts = path.split('/');
        let mut params = HashMap::new();

        while let (Some(route_part), Some(path_part)) = (route_parts.next(), path_parts.next()) {
            if let Some(stripped) = route_part.strip_prefix(':') {
                params.insert(stripped.to_string(), path_part.to_string());
            } else if route_part != path_part {
                return None;
            }
        }

        if route_parts.next().is_some() || path_parts.next().is_some() {
            return None;
        }

        Some(params)
    }
}
