use bytes::{Buf, Bytes};
use std::collections::BTreeMap;

use crate::{tokens, RequestError};

#[derive(Debug, Default)]
pub struct RequestHeaders<'a> {
    pub headers: BTreeMap<&'a str, &'a [u8]>,
}

impl<'a> RequestHeaders<'a> {
    pub fn new() -> Self {
        RequestHeaders {
            headers: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Request<'a> {
    pub method: Option<String>,
    pub path: Option<String>,
    pub version: Option<u8>,
    pub headers: RequestHeaders<'a>,
}

impl<'a> Request<'a> {
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

        // lifetime may not live long enough
        // argument requires that `'1` must outlive `'a`
        // Request::parse_headers(&mut bytes, &mut self.headers)?;

        Ok(())
    }

    pub fn parse_headers(_bytes: &mut Bytes, _headers: &'a mut RequestHeaders<'a>) -> Result<(), RequestError> {
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
