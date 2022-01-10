use super::super::parts::Headers;
use crate::message::parts::HeaderName;

pub trait SetReference {
    /// Reference* header
    fn set_reference(&mut self, index: u16, value: String) -> Option<String>;
}

impl SetReference for Headers {
    fn set_reference(&mut self, index: u16, value: String) -> Option<String> {
        self.insert_by_header_name(HeaderName::Reference(index), value)
    }
}
