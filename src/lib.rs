use std::collections::HashMap;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_request() {
        let request =
            Request::parse("GET SHIORI/3.0\r\nID: AAA\r\nCharset: UTF-8\r\nReference0: foo\\nbar\r\nX-Ray: y\r\n\r\n").unwrap();
        assert_eq!(request.method.to_string(), "GET");
        assert_eq!(request.version.to_string(), "3.0");
        println!("'{}'", request);
        assert_eq!(request.get_header("ID").unwrap().as_str(), "AAA");
        assert_eq!(request.get_header("Charset").unwrap().as_str(), "UTF-8");
        assert_eq!(
            request.get_header("Reference0").unwrap().as_str(),
            "foo\\nbar"
        );
        assert_eq!(request.get_header("X-Ray").unwrap().as_str(), "y");
    }

    #[test]
    fn parse_response() {
        let request =
            Response::parse("SHIORI/3.0 200 OK\r\nID: AAA\r\nCharset: UTF-8\r\nReference0: foo\\nbar\r\nX-Ray: y\r\n\r\n").unwrap();
        assert_eq!(request.status.to_string(), "200 OK");
        assert_eq!(request.version.to_string(), "3.0");
        println!("'{}'", request);
        assert_eq!(request.get_header("ID").unwrap().as_str(), "AAA");
        assert_eq!(request.get_header("Charset").unwrap().as_str(), "UTF-8");
        assert_eq!(
            request.get_header("Reference0").unwrap().as_str(),
            "foo\\nbar"
        );
        assert_eq!(request.get_header("X-Ray").unwrap().as_str(), "y");
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum HeaderName {
    Standard(StandardHeaderName),
    Reference(u8),
    Custom(String),
}

#[derive(Debug)]
pub struct HeaderNameParseError;

impl std::str::FromStr for HeaderName {
    type Err = HeaderNameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(header) = StandardHeaderName::from_str(s) {
            return Ok(HeaderName::Standard(header));
        }
        if s.get(..9) == Some("Reference") {
            if let Ok(index) = u8::from_str(s) {
                return Ok(HeaderName::Reference(index));
            }
        }
        Ok(HeaderName::Custom(s.to_string()))
    }
}

impl HeaderName {
    pub fn from(s: &str) -> Self {
        HeaderName::from_str(s).unwrap()
    }
}

impl std::fmt::Display for HeaderName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeaderName::Standard(header) => f.write_str(header.into()),
            HeaderName::Reference(index) => write!(f, "Reference{}", index),
            HeaderName::Custom(name) => name.fmt(f),
        }
    }
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    Hash,
    strum_macros::EnumString,
    strum_macros::Display,
    strum_macros::IntoStaticStr,
)]
pub enum StandardHeaderName {
    /// Sender header
    Sender,
    /// Charset header
    Charset,
    /// SecurityLevel header (SHIORI/2.2,2.6,3.x)
    SecurityLevel,
    /// ID header (SHIORI/2.5,3.x)
    ID,
    /// Event header (SHIORI/2.2)
    Event,
    /// Type header (GET Word SHIORI/2.0)
    Type,
    /// Status header (SHIORI/3.0 SSP extended)
    Status,
    /// Ghost header (NOTIFY OwnerGhostName SHIORI/2.0,2.3)
    Ghost,
    /// Sentence header (SHIORI/2.0,2.3b)
    Sentence,
    /// To header (SHIORI/2.3b)
    To,
    /// Age header (SHIORI/2.3b)
    Age,
    /// Surface header (SHIORI/2.3b)
    Surface,
    /// Word header (TEACH SHIORI/2.4)
    Word,
}

#[derive(Debug)]
pub struct Headers {
    headers: HashMap<HeaderName, String>,
}

impl Headers {
    pub fn new(headers: HashMap<HeaderName, String>) -> Headers {
        Headers { headers: headers }
    }

    pub fn get(&self, name: &HeaderName) -> Option<&String> {
        self.headers.get(name)
    }

    pub fn insert(&mut self, name: HeaderName, value: String) -> Option<String> {
        self.headers.insert(name, value)
    }

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
        Ok(Headers::new(headers))
    }
}

impl std::fmt::Display for Headers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (key, value) in self.headers.iter() {
            key.fmt(f)?;
            f.write_str(": ")?;
            f.write_str(value)?;
            f.write_str(CRLF)?;
        }
        Ok(())
    }
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    Hash,
    strum_macros::EnumString,
    strum_macros::Display,
    strum_macros::IntoStaticStr,
)]
pub enum Method {
    GET,
    NOTIFY,
    #[strum(serialize = "GET Version")]
    GETVersion,
    #[strum(serialize = "GET Sentence")]
    GETSentence,
    #[strum(serialize = "GET Word")]
    GETWord,
    #[strum(serialize = "GET Status")]
    GETStatus,
    TEACH,
    #[strum(serialize = "GET String")]
    GETString,
    #[strum(serialize = "NOTIFY OwnerGhostName")]
    NOTIFYOwnerGhostName,
    #[strum(serialize = "NOTIFY OtherGhostName")]
    NOTIFYOtherGhostName,
    #[strum(serialize = "TRANSLATE Sentence")]
    TRANSLATESentence,
    TRANSLATE,
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    Hash,
    strum_macros::EnumString,
    strum_macros::Display,
    strum_macros::IntoStaticStr,
)]
pub enum Version {
    #[strum(serialize = "2.0")]
    V20,
    #[strum(serialize = "2.2")]
    V22,
    #[strum(serialize = "2.3")]
    V23,
    #[strum(serialize = "2.4")]
    V24,
    #[strum(serialize = "2.5")]
    V25,
    #[strum(serialize = "2.6")]
    V26,
    #[strum(serialize = "3.0")]
    V30,
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    Hash,
    strum_macros::EnumString,
    strum_macros::Display,
    strum_macros::IntoStaticStr,
    strum_macros::EnumMessage,
)]
pub enum Status {
    #[strum(serialize = "200 OK", message = "OK")]
    OK = 200,
    #[strum(serialize = "204 No Content", message = "No Content")]
    NoContent = 204,
    #[strum(serialize = "310 Communicate", message = "Communicate")]
    Communicate = 310,
    #[strum(serialize = "311 Not Enough", message = "Not Enough")]
    NotEnough = 311,
    #[strum(serialize = "312 Advice", message = "Advice")]
    Advice = 312,
    #[strum(serialize = "400 Bad Request", message = "Bad Request")]
    BadRequest = 400,
    #[strum(
        serialize = "500 Internal Server Error",
        message = "Internal Server Error"
    )]
    InternalServerError = 500,
}

pub const CRLF: &'static str = "\r\n";
pub const CRLFCRLF: &'static str = "\r\n\r\n";

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub version: Version,
    pub headers: Headers,
}

#[derive(Debug)]
pub struct Response {
    pub version: Version,
    pub status: Status,
    pub headers: Headers,
}

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

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} SHIORI/{}{}{}{}",
            self.method, self.version, CRLF, self.headers, CRLF
        )
    }
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "SHIORI/{} {}{}{}{}",
            self.version, self.status, CRLF, self.headers, CRLF
        )
    }
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

trait GetHeader<T> {
    fn get_header(&self, name: T) -> Option<&String>;
}

trait SetHeader<T> {
    fn set_header(&mut self, name: T, value: String) -> Option<String>;
}

impl GetHeader<&str> for Request {
    fn get_header(&self, name: &str) -> Option<&String> {
        self.headers.get(&HeaderName::from(name))
    }
}

impl SetHeader<&str> for Request {
    fn set_header(&mut self, name: &str, value: String) -> Option<String> {
        self.headers.insert(HeaderName::from(name), value)
    }
}

impl GetHeader<&HeaderName> for Request {
    fn get_header(&self, name: &HeaderName) -> Option<&String> {
        self.headers.get(name)
    }
}

impl SetHeader<HeaderName> for Request {
    fn set_header(&mut self, name: HeaderName, value: String) -> Option<String> {
        self.headers.insert(name, value)
    }
}

impl GetHeader<&str> for Response {
    fn get_header(&self, name: &str) -> Option<&String> {
        self.headers.get(&HeaderName::from(name))
    }
}

impl SetHeader<&str> for Response {
    fn set_header(&mut self, name: &str, value: String) -> Option<String> {
        self.headers.insert(HeaderName::from(name), value)
    }
}

impl GetHeader<&HeaderName> for Response {
    fn get_header(&self, name: &HeaderName) -> Option<&String> {
        self.headers.get(name)
    }
}

impl SetHeader<HeaderName> for Response {
    fn set_header(&mut self, name: HeaderName, value: String) -> Option<String> {
        self.headers.insert(name, value)
    }
}
