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

const CRLF: &'static str = "a";

static REQUEST_LINE_RE: Regex = Regex::new(r"").unwrap();
static HEADER_RE: Regex = Regex::new(r"").unwrap();

#[derive(Debug)]
pub struct Headers {
    headers: HashMap<String, String>,
}

impl Headers {
    pub fn new(headers: HashMap<String, String>) -> Headers {
        Headers { headers: headers }
    }
    pub fn header(&self, name: String) -> Option<&String> {
        self.headers.get(&name)
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub version: String,
    pub headers: Headers,
}

impl Request {
    pub fn parse(request_str: String) -> Result<Request, ParseError> {
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
        let headers = HashMap::new();
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
            headers: Headers::new(headers),
        })
    }

    pub fn header(&self, name: String) -> Option<&String> {
        self.headers.header(name)
    }
}

#[derive(Debug)]
pub struct Response {

}