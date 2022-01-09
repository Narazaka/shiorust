use super::header_name::HeaderName;
use crate::common::CRLF;
use std::collections::HashMap;

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
