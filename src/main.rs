use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
};
use tokio::net::TcpListener;
use tokio::{
    fs::{self, create_dir_all, File},
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[derive(PartialEq)]
enum RequestMethod {
    Get,
    Post,
}

struct HttpRequest {
    method: RequestMethod,
    path: String,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpRequest {
    async fn parse_request(stream: &mut TcpStream) -> Option<Self> {
        let mut reader = BufReader::new(stream);
        let mut request_lines = Vec::new();
        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line).await.unwrap();
            if bytes_read == 0 {
                break;
            }
            if line.trim().is_empty() {
                break;
            }
            request_lines.push(line.trim().to_string());
        }

        let first_line = request_lines.first()?;
        let mut parts = first_line.split_whitespace();
        let method_str = parts.next()?.to_string();
        let method = match method_str.as_str() {
            "GET" => RequestMethod::Get,
            "POST" => RequestMethod::Post,
            _ => return None,
        };
        let path = parts.next()?.to_string();

        let mut headers = HashMap::new();
        // Read the request headers
        for line in &request_lines[1..] {
            if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }
        }

        // Read the request body
        let mut body = String::new();
        if method == RequestMethod::Post {
            if let Some(content_length) = headers
                .get("Content-Length")
                .and_then(|len| len.parse::<usize>().ok())
            {
                let mut buf = vec![0u8; content_length];
                reader.read_exact(&mut buf).await.unwrap();
                body = String::from_utf8(buf).unwrap();
            }
        }

        Some(Self {
            method,
            path,
            headers,
            body,
        })
    }
}

struct HttpResponse {
    protocol: String,
    status_code: u32,
    status_text: String,
    body: Option<String>,
    headers: HashMap<String, String>,
    content_type: ContentType,
}

enum ContentType {
    TextPlain,
    ApplicationOctetStream,
    None,
}

impl ContentType {
    fn as_str(&self) -> &str {
        match self {
            ContentType::TextPlain => "text/plain",
            ContentType::ApplicationOctetStream => "application/octet-stream",
            ContentType::None => "",
        }
    }
}

impl HttpResponse {
    fn into_response_string(self) -> String {
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

async fn handle_root_request() -> HttpResponse {
    HttpResponse {
        protocol: "HTTP/1.1".to_string(),
        status_code: 200,
        status_text: "OK".to_string(),
        body: None,
        headers: HashMap::new(),
        content_type: ContentType::None,
    }
}

async fn handle_echo_request(path: &str) -> HttpResponse {
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

async fn handle_user_agent_request(headers: &HashMap<String, String>) -> HttpResponse {
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

async fn handle_not_found_request() -> HttpResponse {
    HttpResponse {
        protocol: "HTTP/1.1".to_string(),
        status_code: 404,
        status_text: "Not Found".to_string(),
        body: None,
        headers: HashMap::new(),
        content_type: ContentType::None,
    }
}

async fn handle_file_request(directory: &str, path: &str) -> HttpResponse {
    let filename = path.trim_start_matches("/files/");
    let mut file_path = PathBuf::from(directory);
    file_path.push(filename);

    match File::open(&file_path).await {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents).await {
                Ok(_) => HttpResponse {
                    protocol: "HTTP/1.1".to_string(),
                    status_code: 200,
                    status_text: "OK".to_string(),
                    body: Some(contents),
                    headers: HashMap::new(),
                    content_type: ContentType::ApplicationOctetStream,
                },
                Err(_) => handle_not_found_request().await,
            }
        }
        Err(_e) => handle_not_found_request().await,
    }
}

async fn handle_post_file_request(directory: &str, path: &str, body: String) -> HttpResponse {
    let filename = path.trim_start_matches("/files/");
    let file_path = Path::new(&directory).join(filename);

    if !file_path.parent().unwrap().exists() {
        if let Err(err) = create_dir_all(file_path.parent().unwrap()).await {
            eprintln!("Error creating parent directories: {}", err);
        }
    }

    if !file_path.exists() {
        if let Err(err) = File::create(&file_path).await {
            eprintln!("Creating new file error: {} - {:?}", err, file_path);
        }
    }

    if let Err(err) = fs::write(&file_path, body).await {
        eprintln!("Error writing file: {}", err);
    }

    HttpResponse {
        protocol: "HTTP/1.1".to_string(),
        status_code: 201,
        status_text: "Created".to_string(),
        body: None,
        headers: HashMap::new(),
        content_type: ContentType::None,
    }
}

async fn handle_request(mut stream: TcpStream, directory: &str) {
    if let Some(http_request) = HttpRequest::parse_request(&mut stream).await {
        let response = match http_request.method {
            RequestMethod::Get => match http_request.path.as_str() {
                path if path.starts_with("/echo/") => handle_echo_request(path).await,
                path if path.starts_with("/files/") => handle_file_request(directory, path).await,
                "/user-agent" => handle_user_agent_request(&http_request.headers).await,
                "/" => handle_root_request().await,
                _ => handle_not_found_request().await,
            },
            RequestMethod::Post => match http_request.path.as_str() {
                path if path.starts_with("/files/") => {
                    handle_post_file_request(directory, path, http_request.body).await
                }
                _ => handle_not_found_request().await,
            },
        };

        if let Err(e) = stream
            .write_all(&response.into_response_string().into_bytes())
            .await
        {
            eprintln!("Failed to write HttpResponse: {}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let port = "4221";
    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();
    println!("Connection to port: {}", port);

    let directory = if let Some(dir) = args.get(2) {
        dir.clone()
    } else {
        eprintln!("No directory argument provided. Using default directory.");
        String::from("default")
    };

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("Retrieved a connection");
                let directory = directory.clone();
                tokio::spawn(async move {
                    handle_request(stream, &directory).await;
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
