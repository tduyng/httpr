use std::collections::HashMap;

pub struct HttpResponse {
    pub protocol: String,
    pub status_code: u32,
    pub status_text: String,
    pub body: Option<String>,
    pub headers: HashMap<String, String>,
    pub content_type: ContentType,
}

pub enum ContentType {
    TextPlain,
    ApplicationOctetStream,
    None,
}

impl ContentType {
    pub fn as_str(&self) -> &str {
        match self {
            ContentType::TextPlain => "text/plain",
            ContentType::ApplicationOctetStream => "application/octet-stream",
            ContentType::None => "",
        }
    }
}

impl HttpResponse {
    pub fn into_response_string(self) -> String {
        let mut response = format!(
            "{} {} {}\r\n",
            self.protocol, self.status_code, self.status_text
        );

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value))
        }

        let content_type_str = self.content_type.as_str();
        if !content_type_str.is_empty() {
            response.push_str(&format!("Content-Type: {}\r\n", content_type_str));
        }

        if let Some(body) = &self.body {
            response.push_str(&format!("Content-Length: {}\r\n\r\n", body.len()));
            response.push_str(body);
        } else {
            response.push_str("\r\n");
        }

        response
    }
}
