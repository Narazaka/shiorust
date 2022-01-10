use crate::message::parts::{HeaderName, Headers};

pub trait Entry<T> {
    fn entry(&mut self, name: T) -> std::collections::hash_map::Entry<HeaderName, String>;
}

impl Entry<&str> for Headers {
    fn entry(&mut self, name: &str) -> std::collections::hash_map::Entry<HeaderName, String> {
        self.entry_by_header_name(HeaderName::from(name))
    }
}

impl Entry<HeaderName> for Headers {
    fn entry(&mut self, name: HeaderName) -> std::collections::hash_map::Entry<HeaderName, String> {
        self.entry_by_header_name(name)
    }
}
