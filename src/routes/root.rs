use std::collections::HashMap;

use crate::response::{ContentType, HttpResponse};

pub async fn handle_root_request() -> HttpResponse {
    HttpResponse {
        protocol: "HTTP/1.1".to_string(),
        status_code: 200,
        status_text: "OK".to_string(),
        body: None,
        headers: HashMap::new(),
        content_type: ContentType::None,
    }
}
