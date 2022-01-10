use super::header_name::HeaderName;
use crate::common::CRLF;
use std::collections::hash_map::*;
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

    #[test]
    fn test_format() {
        let mut h = Headers::new();
        h.insert_by_header_name(HeaderName::Reference(0), "foo".to_string());
        assert_eq!(format!("{}", h), "Reference0: foo\r\n");
    }
}

#[derive(Debug)]
pub struct Headers(HashMap<HeaderName, String>);

impl Headers {
    pub fn new() -> Headers {
        Headers(HashMap::new())
    }

    pub fn from_hashmap(headers: HashMap<HeaderName, String>) -> Headers {
        Headers(headers)
    }

    pub fn entry_by_header_name(&mut self, key: HeaderName) -> Entry<HeaderName, String> {
        self.0.entry(key)
    }

    pub fn get_by_header_name(&self, name: &HeaderName) -> Option<&String> {
        self.0.get(name)
    }

    pub fn insert_by_header_name(&mut self, name: HeaderName, value: String) -> Option<String> {
        self.0.insert(name, value)
    }

    pub fn names(&self) -> Keys<HeaderName, String> {
        self.0.keys()
    }

    pub fn iter(&self) -> Iter<HeaderName, String> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<HeaderName, String> {
        self.0.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl std::fmt::Display for Headers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (key, value) in self.0.iter() {
            key.fmt(f)?;
            f.write_str(": ")?;
            f.write_str(value)?;
            f.write_str(CRLF)?;
        }
        Ok(())
    }
}

impl std::ops::Deref for Headers {
    type Target = HashMap<HeaderName, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
