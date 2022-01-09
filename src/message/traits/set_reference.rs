use super::SetHeader;
use crate::message::parts::HeaderName;

pub trait SetReference {
    fn set_reference(&mut self, index: u16, value: String) -> Option<String>;
}

impl<T> SetReference for T
where
    T: SetHeader<HeaderName>,
{
    fn set_reference(&mut self, index: u16, value: String) -> Option<String> {
        self.set_header(HeaderName::Reference(index), value)
    }
}
