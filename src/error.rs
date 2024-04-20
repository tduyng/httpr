use std::{error::Error, fmt, io};

use crate::response::Response;

#[derive(Debug)]
pub enum ServerError {
    IoError(io::Error),
    ParseError(String),
    NotFound,
    InternalError(String),
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
            ServerError::IoError(err) => write!(f, "IO error: {}", err),
            ServerError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ServerError::InternalError(msg) => write!(f, "Stream error: {}", msg),
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
