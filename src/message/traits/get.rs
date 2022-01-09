use crate::message::parts::{HeaderName, Headers};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_headers() {
        let mut h = Headers::new();
        h.insert_by_header_name(HeaderName::Reference(0), "foo".to_string());
        assert_eq!(h.get(&HeaderName::Reference(0)), Some(&"foo".to_string()));
        assert_eq!(h.get("Reference0"), Some(&"foo".to_string()));
    }
}

pub trait Get<T> {
    fn get(&self, name: T) -> Option<&String>;
}

impl Get<&str> for Headers {
    fn get(&self, name: &str) -> Option<&String> {
        self.get_by_header_name(&HeaderName::from(name))
    }
}

impl Get<&HeaderName> for Headers {
    fn get(&self, name: &HeaderName) -> Option<&String> {
        self.get_by_header_name(name)
    }
}
