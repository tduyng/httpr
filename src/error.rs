use std::{error::Error, fmt, io};

use crate::response::Response;

#[derive(Debug)]
pub enum ServerError {
    IoError(io::Error),
    FmtError(fmt::Error),
    ParseError(String),
    NotFound,
    Unauthorized,
    Forbidden,
    BadGateway,
    StreamError(String),
}

impl From<io::Error> for ServerError {
    fn from(err: io::Error) -> Self {
        ServerError::IoError(err)
    }
}

impl Error for ServerError {}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::NotFound => write!(f, "404 Not Found"),
            ServerError::Unauthorized => write!(f, "401 Unauthorized"),
            ServerError::Forbidden => write!(f, "403 Forbidden"),
            ServerError::BadGateway => write!(f, "502 Bad Gateway"),
            ServerError::IoError(err) => write!(f, "IO error: {}", err),
            ServerError::FmtError(err) => write!(f, "Fmt error: {}", err),
            ServerError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ServerError::StreamError(msg) => write!(f, "Stream error: {}", msg),
        }
    }
}

pub fn internal_server_error() -> Response {
    Response::new()
        .body_str("Internal Server Error")
        .status_code(500, "Internal Server Error")
        .header("Content-Type", "text/plain")
}

pub fn not_found() -> Response {
    Response::new()
        .body_str("404 Not Found")
        .status_code(404, "Not Found")
        .header("Content-Type", "text/plain")
}

pub fn unauthorized() -> Response {
    Response::new()
        .body_str("401 Unauthorized")
        .status_code(401, "Unauthorized")
        .header("Content-Type", "text/plain")
}

pub fn forbidden() -> Response {
    Response::new()
        .body_str("403 Forbidden")
        .status_code(403, "Forbidden")
        .header("Content-Type", "text/plain")
}

pub fn bad_gateway() -> Response {
    Response::new()
        .body_str("502 Bad Gateway")
        .status_code(502, "Bad Gateway")
        .header("Content-Type", "text/plain")
}
