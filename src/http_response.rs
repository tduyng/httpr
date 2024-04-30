use bytes::{BufMut, BytesMut};
use httpstatus::StatusCode;
use std::collections::BTreeMap;

pub struct HttpResponse {
    status_code: StatusCode,
    content_type: String,
    headers: BTreeMap<String, String>,
    body: BytesMut,
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            status_code: StatusCode::Ok,
            content_type: "text/plain".to_string(),
            headers: BTreeMap::new(),
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

    pub fn set_header(&mut self, key: &str, value: &str) -> Option<()> {
        match self.headers.insert(key.to_string(), value.to_string()) {
            Some(_) => Some(()),
            _ => None,
        }
    }

    pub fn build(&self) -> Vec<u8> {
        let mut response = b"HTTP/1.1 ".to_vec();

        response.put_slice(self.status_code.as_u16().to_string().as_bytes());
        response.put_slice(b" ");
        response.put(self.status_code.reason_phrase().as_bytes());
        response.put_slice(b"\r\n");

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
            response.put_slice(b"\r\n");
        }
        response.put_slice(b"\r\n");

        // parse body
        let body = self.body.clone();
        response.put(body);

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_basic_response() {
        let mut response = HttpResponse::new();
        response.write(b"hi");
        response.set_header("x-some-test-header", "some-value");
        assert_eq!(
            response.build(),
            b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nContent-Type: text/plain\r\nx-some-test-header: some-value\r\n\r\nhi"
        )
    }
}
