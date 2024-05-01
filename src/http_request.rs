use bytes::{Buf, Bytes};
use std::collections::{btree_map, BTreeMap};

use crate::{tokens, RequestError};

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
}

#[derive(Debug, Default)]
pub struct Request {
    pub method: Option<String>,
    pub path: Option<String>,
    pub version: Option<u8>,
    pub headers: RequestHeaders,
}

impl Request {
    pub fn new() -> Self {
        Request {
            method: None,
            path: None,
            version: None,
            headers: RequestHeaders::new(),
        }
    }

    pub fn parse(&mut self, buf: Bytes) -> Result<(), RequestError> {
        let mut bytes = buf;
        self.method = Some(Request::parse_token(&mut bytes)?);
        self.path = Some(Request::parse_uri(&mut bytes)?);
        self.version = Some(Request::parse_version(&mut bytes)?);
        Request::parse_new_line(&mut bytes)?;
        Request::parse_headers(&mut bytes, &mut self.headers)?;

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
        assert_eq!(request.method, Some(String::from("GET")));
        assert_eq!(request.path, Some(String::from("/test")));
    }

    #[test]
    fn accept_only_newline() {
        let mut request = Request::new();

        request
            .parse(Bytes::from_static(b"GET /test HTTP/1.1\n"))
            .expect("parsing request");

        assert_eq!(request.version, Some(1));
        assert_eq!(request.method, Some(String::from("GET")));
        assert_eq!(request.path, Some(String::from("/test")));
    }

    #[test]
    fn do_not_accept_only_cr() {
        let mut request = Request::new();

        request
            .parse(Bytes::from_static(b"GET /test HTTP/1.1\r"))
            .expect_err("parsing request");
    }
}
