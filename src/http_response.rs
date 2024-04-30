use std::collections::HashMap;

use bytes::{BufMut, BytesMut};
use httpstatus::StatusCode;

pub struct HttpResponse {
    status_code: StatusCode,
    content_type: String,
    headers: HashMap<String, String>,
    body: BytesMut,
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            status_code: StatusCode::Ok,
            content_type: "text/plain".to_string(),
            headers: HashMap::new(),
            body: BytesMut::new(),
        }
    }
}

impl From<HttpResponse> for Vec<u8> {
    fn from(builder: HttpResponse) -> Self {
        builder.build()
    }
}

impl HttpResponse {
    pub fn new() -> Self {
        HttpResponse { ..Default::default() }
    }

    pub fn status_code(&mut self, status: StatusCode) -> &mut Self {
        self.status_code = status;
        self
    }

    pub fn content_type(&mut self, content_type: String) -> &mut Self {
        self.content_type = content_type;
        self
    }

    pub fn write(&mut self, src: &[u8]) {
        self.body.put_slice(src)
    }

    pub fn build(&self) -> Vec<u8> {
        let mut response = b"HTTP/1.1 ".to_vec();

        response.put_slice(self.status_code.as_u16().to_string().as_bytes());
        response.put_slice(b" ");
        response.put(self.status_code.reason_phrase().as_bytes());
        response.put_slice(b"\n");

        // parse headers
        let mut headers = self.headers.clone();
        let content_type = if !self.content_type.is_empty() {
            self.content_type.clone()
        } else {
            "text/plain".to_string()
        };
        let content_length = self.body.len();
        headers.insert("Content-Type".to_string(), content_type);
        headers.insert("Content-Length".to_string(), content_length.to_string());
        for (key, val) in &headers {
            response.put_slice(key.as_bytes());
            response.put_slice(b": ");
            response.put_slice(val.as_bytes());
            response.put_slice(b"\n");
        }
        response.put_slice(b"\n");

        // parse body
        let body = self.body.clone();
        response.put(body);

        response
    }
}
