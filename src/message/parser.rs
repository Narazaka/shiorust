use crate::common::{CRLF, CRLFCRLF};
use crate::message::parts::*;
use crate::message::{Request, Response};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
}

#[derive(Debug)]
pub enum ParseErrorKind {
    InvalidLength,
    TerminateNotFound,
    RequestLine,
    StatusLine,
    HeaderLine,
}

pub trait Parser<T, U> {
    fn parse(payload: &str) -> Result<T, ParseError>;

    fn parse_initial_line(line: &str) -> Result<U, ParseError>;

    fn parse_general(payload: &str) -> Result<(U, Headers), ParseError> {
        let end_index = payload.rfind(CRLFCRLF).ok_or(ParseError {
            kind: ParseErrorKind::TerminateNotFound,
        })?;
        let mut lines = payload[..end_index].split(CRLF);
        let line = lines.next().ok_or(ParseError {
            kind: ParseErrorKind::InvalidLength,
        })?;
        let initial_line = Self::parse_initial_line(line)?;
        let headers = Headers::parse_headers(lines)?;
        Ok((initial_line, headers))
    }
}

impl Parser<Request, (Method, Version)> for Request {
    fn parse(request_str: &str) -> Result<Request, ParseError> {
        let ((method, version), headers) = Self::parse_general(request_str)?;
        Ok(Request {
            method,
            version,
            headers,
        })
    }

    fn parse_initial_line(request_line: &str) -> Result<(Method, Version), ParseError> {
        if let Some(index) = request_line.find(" SHIORI/") {
            let method = &request_line[..index];
            let version = &request_line[index + 8..];
            if let (Ok(method), Ok(version)) =
                (Method::from_str(method), Version::from_str(version))
            {
                return Ok((method, version));
            }
        }
        return Err(ParseError {
            kind: ParseErrorKind::RequestLine,
        });
    }
}

impl Parser<Response, (Version, Status)> for Response {
    fn parse(request_str: &str) -> Result<Response, ParseError> {
        let ((version, status), headers) = Self::parse_general(request_str)?;
        Ok(Response {
            version,
            status,
            headers,
        })
    }
    fn parse_initial_line(status_line: &str) -> Result<(Version, Status), ParseError> {
        if status_line.len() < 14
            || status_line.find("SHIORI/") != Some(0)
            || status_line[7..].find(' ') != Some(3)
        //  || status_line[11..].find(' ') != Some(3)
        {
            return Err(ParseError {
                kind: ParseErrorKind::StatusLine,
            });
        }
        let version = Version::from_str(&status_line[7..10]).or(Err(ParseError {
            kind: ParseErrorKind::StatusLine,
        }))?;
        let status = Status::from_str(&status_line[11..]).or(Err(ParseError {
            kind: ParseErrorKind::StatusLine,
        }))?;
        Ok((version, status))
    }
}

impl Headers {
    pub fn parse_headers(header_lines: std::str::Split<&str>) -> Result<Headers, ParseError> {
        let mut headers = HashMap::new();
        for header_line in header_lines {
            let index = header_line.find(": ").ok_or(ParseError {
                kind: ParseErrorKind::HeaderLine,
            })?;
            let key = &header_line[..index];
            let value = &header_line[index + 2..];
            headers.insert(HeaderName::from(key), value.to_string());
        }
        Ok(Headers::from_hashmap(headers))
    }
}
