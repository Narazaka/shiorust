#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;

use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
}

#[derive(Debug)]
pub enum ParseErrorKind {
    InvalidLength,
    RequestLine,
    HeaderLine,
}

const CRLF: &'static str = "\r\n";
lazy_static! {
    static ref REQUEST_LINE_RE: Regex = Regex::new(r"").unwrap();
    static ref HEADER_RE: Regex = Regex::new(r"").unwrap();
}

#[derive(Debug)]
pub struct Headers<'a> {
    headers: &'a HashMap<&'a str, &'a str>,
}

impl<'a> Headers<'a> {
    pub fn new(headers: &'a HashMap<&str, &str>) -> Headers<'a> {
        Headers { headers: headers }
    }
    pub fn header(&self, name: &str) -> Option<&&str> {
        self.headers.get(name)
    }
}

#[derive(Debug)]
pub struct Request<'a> {
    pub method: &'a str,
    pub version: &'a str,
    pub headers: Headers<'a>,
}

impl<'a> Request<'a> {
    pub fn parse(request_str: &str) -> Result<Request, ParseError> {
        use self::ParseErrorKind::*;

        let lines = request_str.split(CRLF);
        let (method, version) = if let Some(request_line) = lines.take(1).next() {
            if let Some(captures) = REQUEST_LINE_RE.captures(request_line) {
                (captures.get(1).unwrap().as_str(), captures.get(2).unwrap().as_str())
            } else {
                return Err(ParseError { kind: RequestLine });
            }
        } else {
            return Err(ParseError { kind: InvalidLength });
        };
        let mut headers = HashMap::new();
        let header_lines = lines.skip(1);
        for header_line in header_lines {
            if let Some(captures) = HEADER_RE.captures(header_line) {
                headers.insert(captures.get(1).unwrap().as_str(),
                               captures.get(2).unwrap().as_str());
            } else {
                return Err(ParseError { kind: HeaderLine });
            }
        }
        Ok(Request {
            method: method,
            version: version,
            headers: Headers::new(&headers),
        })
    }

    pub fn header(&self, name: &str) -> Option<&&str> {
        self.headers.header(name)
    }
}

#[derive(Debug)]
pub struct Response {

}