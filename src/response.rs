use bytes::{BufMut, BytesMut};
use httpstatus::StatusCode;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    content_type: String,
    headers: BTreeMap<String, String>,
    body: BytesMut,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status_code: StatusCode::Ok,
            content_type: "text/plain".to_string(),
            headers: BTreeMap::new(),
            body: BytesMut::new(),
        }
    }
}

impl From<Response> for Vec<u8> {
    fn from(builder: Response) -> Self {
        builder.build()
    }
}

impl Response {
    pub fn new() -> Self {
        Response { ..Default::default() }
    }

    pub fn status_code(&mut self, status: StatusCode) -> &mut Self {
        self.status_code = status;
        self
    }

    pub fn content_type(&mut self, content_type: &str) -> &mut Self {
        self.content_type = content_type.to_string();
        self
    }

    pub fn write(&mut self, src: &[u8]) {
        self.body.put_slice(src)
    }

    pub fn clear(&mut self) {
        self.body.clear()
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
        let body = self.body.clone();
        let content_length = body.len();

        let content_type = if !self.content_type.is_empty() {
            self.content_type.clone()
        } else {
            "text/plain".to_string()
        };

        let mut headers = self.headers.clone();
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
        let mut response = Response::new();
        response.write(b"hi");
        response.set_header("x-some-test-header", "some-value");
        assert_eq!(
            response.build(),
            b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nContent-Type: text/plain\r\nx-some-test-header: some-value\r\n\r\nhi"
        )
    }

    #[test]
    fn empty_response() {
        let response = Response::new();
        println!("{}", std::str::from_utf8(&response.build()).unwrap());
        assert_eq!(
            response.build(),
            b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nContent-Type: text/plain\r\n\r\n"
        )
    }

    #[test]
    fn response_with_different_status_code() {
        let mut response = Response::new();
        response.status_code(StatusCode::NotFound);
        assert_eq!(
            response.build(),
            b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nContent-Type: text/plain\r\n\r\n"
        )
    }

    #[test]
    fn response_with_different_content_type() {
        let mut response = Response::new();
        response.content_type("application/json");
        assert_eq!(
            response.build(),
            b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nContent-Type: application/json\r\n\r\n"
        )
    }

    #[test]
    fn response_with_custom_headers() {
        let mut response = Response::new();
        response.set_header("x-custom-header", "custom-value");
        assert_eq!(
               response.build(),
               b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nContent-Type: text/plain\r\nx-custom-header: custom-value\r\n\r\n"
           )
    }

    #[test]
    fn response_with_body_content() {
        let mut response = Response::new();
        response.write(b"hello");
        assert_eq!(
            response.build(),
            b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\nContent-Type: text/plain\r\n\r\nhello"
        )
    }

    #[test]
    fn modify_response_body() {
        let mut response = Response::new();
        response.write(b"hello");
        response.write(b" world");
        assert_eq!(
            response.build(),
            b"HTTP/1.1 200 OK\r\nContent-Length: 11\r\nContent-Type: text/plain\r\n\r\nhello world"
        )
    }

    #[test]
    fn clear_response_body() {
        let mut response = Response::new();
        response.write(b"hello");
        response.clear();
        assert_eq!(
            response.build(),
            b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nContent-Type: text/plain\r\n\r\n"
        )
    }
}
