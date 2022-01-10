use super::parts::{Headers, Method, Version};
use crate::common::CRLF;

/// SHIORI Request message
#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub version: Version,
    pub headers: Headers,
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
