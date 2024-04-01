use std::collections::HashMap;

use crate::response::{ContentType, HttpResponse};

pub async fn handle_echo_request(path: &str) -> HttpResponse {
    let text = path.trim_start_matches("/echo/");
    HttpResponse {
        protocol: "HTTP/1.1".to_string(),
        status_code: 200,
        status_text: "OK".to_string(),
        body: Some(text.to_string()),
        headers: HashMap::new(),
        content_type: ContentType::TextPlain,
    }
}
