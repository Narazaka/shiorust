use super::super::parts::Headers;
use crate::message::parts::HeaderName;

pub trait GetReference {
    /// Reference* header
    fn get_reference(&self, index: u16) -> Option<&String>;
}

impl GetReference for Headers {
    fn get_reference(&self, index: u16) -> Option<&String> {
        self.get_by_header_name(&HeaderName::Reference(index))
    }
}
