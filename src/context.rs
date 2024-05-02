use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::Request;
use crate::Response;

pub struct Context {
    pub request: Arc<Mutex<Request>>,
    pub response: Arc<Mutex<Response>>,
    pub path: String,
    pub path_params: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
}

impl Context {
    pub fn new(request: Request, response: Response) -> Self {
        Context {
            request: Arc::new(Mutex::new(request)),
            response: Arc::new(Mutex::new(response)),
            path: "".to_string(),
            path_params: HashMap::new(),
            query_params: HashMap::new(),
        }
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn set_path_params(&mut self, params: HashMap<String, String>) {
        self.path_params = params;
    }

    pub fn set_query_params(&mut self, query_params: HashMap<String, String>) {
        self.query_params = query_params;
    }
}
