use std::collections::HashMap;

use crate::response::{ContentType, HttpResponse};

pub async fn handle_user_agent_request(headers: &HashMap<String, String>) -> HttpResponse {
    let user_agent = headers.get("User-Agent").cloned().unwrap_or_default();
    HttpResponse {
        protocol: "HTTP/1.1".to_string(),
        status_code: 200,
        status_text: "OK".to_string(),
        body: Some(user_agent),
        headers: HashMap::new(),
        content_type: ContentType::TextPlain,
    }
}
