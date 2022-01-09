use super::header_name::HeaderName;
use crate::common::CRLF;
use std::collections::HashMap;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_header_name() {
        let mut h = Headers::new();
        h.insert_by_header_name(HeaderName::Reference(0), "foo".to_string());
        assert_eq!(
            h.get_by_header_name(&HeaderName::Reference(0)),
            Some(&"foo".to_string())
        );
    }
}

#[derive(Debug)]
pub struct Headers {
    headers: HashMap<HeaderName, String>,
}

impl Headers {
    pub fn new() -> Headers {
        Headers {
            headers: HashMap::new(),
        }
    }
    pub fn from_hashmap(headers: HashMap<HeaderName, String>) -> Headers {
        Headers { headers: headers }
    }

    pub fn get_by_header_name(&self, name: &HeaderName) -> Option<&String> {
        self.headers.get(name)
    }

    pub fn insert_by_header_name(&mut self, name: HeaderName, value: String) -> Option<String> {
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
