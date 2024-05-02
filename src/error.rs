use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("invalid method")]
    Method,
    #[error("invalid header name")]
    HeaderName,
    #[error("invalid header value")]
    HeaderValue,
    #[error("invalid header body length")]
    HeaderContentLength,
    #[error("incomplete body")]
    IncompleteBody,
    #[error("invalid status")]
    Status,
    #[error("invalid version")]
    Version,
    #[error("expected newline")]
    NewLine,
    #[error("expected space")]
    Space,
    #[error("invalid token")]
    Token,
    #[error("invalid uri")]
    URI,
    #[error("too many headers")]
    TooManyHeaders,
}

#[derive(Error, Debug)]
pub enum HeaderError {
    #[error("header not found")]
    NotFound,
    #[error("header value is not a valid string")]
    InvalidString,
}
