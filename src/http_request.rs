use bytes::{Buf, Bytes};
use core::fmt;
use std::collections::{btree_map, BTreeMap};

use crate::{tokens, HeaderError, RequestError};

#[derive(Debug, Default)]
pub struct RequestHeaders {
    headers: BTreeMap<String, Vec<u8>>,
}

impl RequestHeaders {
    pub fn new() -> Self {
        RequestHeaders {
            headers: BTreeMap::new(),
        }
    }

    pub fn iter(&self) -> btree_map::Iter<String, Vec<u8>> {
        self.headers.iter()
    }

    pub fn get_str(&self, key: &str) -> Result<String, HeaderError> {
        let header = self.get(key)?;
        String::from_utf8(header.to_vec()).map_err(|_| HeaderError::InvalidString)
    }

    pub fn get(&self, key: &str) -> Result<&Vec<u8>, HeaderError> {
        self.headers.get(key).ok_or(HeaderError::NotFound)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Method {
    OPTION,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    TRACE,
    CONNECT,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method_str = match self {
            Method::OPTION => "OPTIONS",
            Method::GET => "GET",
            Method::HEAD => "HEAD",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::TRACE => "TRACE",
            Method::CONNECT => "CONNECT",
        };
        write!(f, "{}", method_str)
    }
}

impl TryFrom<&str> for Method {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase().as_str() {
            "OPTION" => Ok(Method::OPTION),
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "TRACE" => Ok(Method::TRACE),
            "CONNECT" => Ok(Method::CONNECT),
            _ => Err("invalid method"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Request {
    pub method: Option<Method>,
    pub path: Option<String>,
    pub version: Option<u8>,
    pub headers: RequestHeaders,
    pub body: Vec<u8>,
}

impl Request {
    pub fn new() -> Self {
        Request {
            method: None,
            path: None,
            version: None,
            headers: RequestHeaders::new(),
            body: vec![],
        }
    }

    pub fn parse(&mut self, buf: Bytes) -> Result<(), RequestError> {
        let mut bytes = buf;
        self.method = Some(
            Request::parse_token(&mut bytes)?
                .as_str()
                .try_into()
                .map_err(|_| RequestError::Method)?,
        );
        self.path = Some(Request::parse_uri(&mut bytes)?);
        self.version = Some(Request::parse_version(&mut bytes)?);
        Request::parse_new_line(&mut bytes)?;
        Request::parse_headers(&mut bytes, &mut self.headers)?;
        Request::parse_body(&mut bytes, &self.headers)?;

        Ok(())
    }

    pub fn parse_headers(bytes: &mut Bytes, headers: &mut RequestHeaders) -> Result<(), RequestError> {
        let mut parse_header = || -> Result<(), RequestError> {
            let header_name = Request::parse_header_name(bytes)?;
            Request::parse_space(bytes)?;
            let header_value = Request::parse_header_value(bytes)?;
            headers.headers.insert(header_name, header_value);
            Ok(())
        };

        loop {
            if parse_header().is_err() {
                break;
            }
        }

        Ok(())
    }

    fn parse_new_line(bytes: &mut Bytes) -> Result<(), RequestError> {
        if !bytes.has_remaining() {
            return Err(RequestError::NewLine);
        }

        match bytes.get_u8() {
            b'\r' => {
                if bytes.has_remaining() && bytes.get_u8() == b'\n' {
                    Ok(())
                } else {
                    Err(RequestError::NewLine)
                }
            }
            b'\n' => Ok(()),
            _ => Err(RequestError::NewLine),
        }
    }

    fn parse_version(bytes: &mut Bytes) -> Result<u8, RequestError> {
        let res = match &bytes.slice(0..8)[..] {
            b"HTTP/1.1" => Ok(1),
            b"HTTP/2" => Ok(2),
            b"HTTP/3" => Ok(3),
            _ => return Err(RequestError::Version),
        };
        bytes.advance(8);
        res
    }

    fn parse_uri(bytes: &mut Bytes) -> Result<String, RequestError> {
        for (i, b) in bytes.iter().enumerate() {
            if b == &b' ' {
                let token = &bytes.slice(0..i)[..];
                bytes.advance(i + 1);
                return Ok(std::str::from_utf8(token).map_err(|_| RequestError::URI)?.to_string());
            } else if !tokens::is_uri_token(*b) {
                break;
            }
        }
        Err(RequestError::URI)
    }

    fn parse_token(bytes: &mut Bytes) -> Result<String, RequestError> {
        for (i, b) in bytes.iter().enumerate() {
            if b == &b' ' {
                let token = &bytes.slice(0..i)[..];
                bytes.advance(i + 1);
                return Ok(std::str::from_utf8(token).map_err(|_| RequestError::Token)?.to_string());
            } else if !tokens::is_token(*b) {
                println!("{}", b);
                break;
            }
        }
        Err(RequestError::Token)
    }

    fn parse_space(bytes: &mut Bytes) -> Result<(), RequestError> {
        if !bytes.has_remaining() {
            return Err(RequestError::NewLine);
        }
        if bytes.get_u8() == b' ' && bytes.has_remaining() {
            Ok(())
        } else {
            Err(RequestError::Space)
        }
    }

    fn parse_header_name(bytes: &mut Bytes) -> Result<String, RequestError> {
        for (i, b) in bytes.iter().enumerate() {
            if b == &b':' {
                let token = &bytes.slice(0..i)[..];
                bytes.advance(i + 1);
                return Ok(std::str::from_utf8(token).map_err(|_| RequestError::Token)?.to_string());
            } else if !tokens::is_header_name_token(*b) {
                break;
            }
        }
        Err(RequestError::Token)
    }

    fn parse_header_value(bytes: &mut Bytes) -> Result<Vec<u8>, RequestError> {
        for (i, b) in bytes.iter().enumerate() {
            if b == &b'\r' || b == &b'\n' {
                let token = &bytes.slice(0..i)[..];
                bytes.advance(i);
                Request::parse_new_line(bytes)?;
                return Ok(token.to_vec());
            } else if !tokens::is_header_value_token(*b) {
                break;
            }
        }
        Err(RequestError::Token)
    }

    fn parse_body(bytes: &mut Bytes, headers: &RequestHeaders) -> Result<Vec<u8>, RequestError> {
        if let Some(content_length) = headers.headers.get("Content-Length") {
            let content_length = std::str::from_utf8(content_length).map_err(|_| RequestError::HeaderContentLength)?;
            let content_length: usize = content_length.parse().map_err(|_| RequestError::HeaderContentLength)?;

            if bytes.remaining() < content_length {
                return Err(RequestError::IncompleteBody);
            }

            Request::parse_new_line(bytes)?;
            let body = bytes[..content_length].to_vec();
            bytes.advance(content_length);
            return Ok(body);
        }

        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::*;

    #[test]
    fn parse_basic_requests() {
        let mut request = Request::new();

        request
            .parse(Bytes::from_static(b"GET /test HTTP/1.1\r\n\r\n"))
            .expect("parsing request");

        assert_eq!(request.version, Some(1));
        assert_eq!(request.method, Some(Method::GET));
        assert_eq!(request.path, Some(String::from("/test")));
    }

    #[test]
    fn accept_only_newline() {
        let mut request = Request::new();

        request
            .parse(Bytes::from_static(b"GET /test HTTP/1.1\r\n\r\n"))
            .expect("parsing request");

        assert_eq!(request.version, Some(1));
        assert_eq!(request.method, Some(Method::GET));
        assert_eq!(request.path, Some(String::from("/test")));
    }

    #[test]
    fn do_not_accept_only_cr() {
        let mut request = Request::new();

        request
            .parse(Bytes::from_static(b"GET /test HTTP/1.1\r"))
            .expect_err("parsing request");
    }

    #[test]
    fn parse_request_with_headers() {
        let mut request = Request::new();

        request
            .parse(Bytes::from_static(
                b"GET /test HTTP/1.1\r\nContent-Type: application/json\r\nAuthorization: Bearer token\r\n\r\n",
            ))
            .expect("parsing request");

        assert_eq!(request.version, Some(1));
        assert_eq!(request.method, Some(Method::GET));
        assert_eq!(request.path, Some(String::from("/test")));
        assert_eq!(request.headers.iter().count(), 2);
        assert_eq!(
            request.headers.iter().find(|(k, _)| k.as_str() == "Content-Type"),
            Some((&String::from("Content-Type"), &b"application/json"[..].to_vec()))
        );
        assert_eq!(
            request.headers.iter().find(|(k, _)| k.as_str() == "Authorization"),
            Some((&String::from("Authorization"), &b"Bearer token"[..].to_vec()))
        );
    }

    #[test]
    fn parse_request_with_body() {
        let mut request = Request::new();

        request
            .parse(Bytes::from_static(
                b"POST /test HTTP/1.1\r\nContent-Length: 11\r\n\r\nHello World",
            ))
            .expect("parsing request");

        assert_eq!(request.version, Some(1));
        assert_eq!(request.method, Some(Method::POST));
        assert_eq!(request.path, Some(String::from("/test")));
        assert_eq!(request.headers.iter().count(), 1);
        assert_eq!(
            request.headers.iter().find(|(k, _)| k.as_str() == "Content-Length"),
            Some((&String::from("Content-Length"), &b"11"[..].to_vec()))
        );
    }

    #[test]
    fn parse_request_with_invalid_method() {
        let mut request = Request::new();

        _ = request
            .parse(Bytes::from_static(b"INVALID /test HTTP/1.1\r\n\r\n"))
            .is_err();
    }

    #[test]
    fn parse_request_with_invalid_uri() {
        let mut request = Request::new();

        _ = request
            .parse(Bytes::from_static(b"GET /test!@# HTTP/1.1\r\n\r\n"))
            .is_err();
    }

    #[test]
    fn parse_request_with_invalid_version() {
        let mut request = Request::new();

        _ = request
            .parse(Bytes::from_static(b"GET /test HTTP/1.0\r\n\r\n"))
            .is_err();
    }
}
