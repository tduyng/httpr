use httpstatus::StatusCode;

pub struct HttpResponse {
    status_code: StatusCode,
    content_type: String,
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            status_code: StatusCode::Ok,
            content_type: "text/plain".to_string(),
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
        HttpResponse {
            ..Default::default()
        }
    }

    pub fn status_code(&mut self, status: StatusCode) -> &mut Self {
        self.status_code = status;
        self
    }

    pub fn content_type(&mut self, content_type: String) -> &mut Self {
        self.content_type = content_type;
        self
    }

    pub fn build(&self) -> Vec<u8> {
        let mut response = b"HTTP/1.1 ".to_vec();

        response.extend(self.status_code.as_u16().to_string().as_bytes());
        response.extend(b" ");
        response.extend(self.status_code.reason_phrase().as_bytes());
        response.extend(b"\n");

        response
    }
}
