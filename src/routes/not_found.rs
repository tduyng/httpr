use std::collections::HashMap;

use crate::response::{ContentType, HttpResponse};

pub async fn handle_not_found_request() -> HttpResponse {
    HttpResponse {
        protocol: "HTTP/1.1".to_string(),
        status_code: 404,
        status_text: "Not Found".to_string(),
        body: None,
        headers: HashMap::new(),
        content_type: ContentType::None,
    }
}
