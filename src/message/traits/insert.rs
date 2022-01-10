use crate::message::parts::{HeaderName, Headers};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_headers() {
        let mut h = Headers::new();
        h.insert(HeaderName::Reference(0), "foo".to_string());
        assert_eq!(
            h.get_by_header_name(&HeaderName::Reference(0)),
            Some(&"foo".to_string())
        );
        h.insert("Reference0", "bar".to_string());
        assert_eq!(
            h.get_by_header_name(&HeaderName::Reference(0)),
            Some(&"bar".to_string())
        );
    }
}

pub trait Insert<T> {
    /// set header
    fn insert(&mut self, name: T, value: String) -> Option<String>;
}

impl Insert<&str> for Headers {
    fn insert(&mut self, name: &str, value: String) -> Option<String> {
        self.insert_by_header_name(HeaderName::from(name), value)
    }
}

impl Insert<HeaderName> for Headers {
    fn insert(&mut self, name: HeaderName, value: String) -> Option<String> {
        self.insert_by_header_name(name, value)
    }
}
