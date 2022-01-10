use super::parts::{Headers, Status, Version};
use crate::common::CRLF;

/// SHIORI Response message
#[derive(Debug)]
pub struct Response {
    pub version: Version,
    pub status: Status,
    pub headers: Headers,
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
