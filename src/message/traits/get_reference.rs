use super::GetHeader;
use crate::message::parts::HeaderName;

pub trait GetReference {
    fn get_reference(&self, index: u8) -> Option<&String>;
}

impl<T> GetReference for T
where
    for<'a> T: GetHeader<&'a HeaderName>,
{
    fn get_reference(&self, index: u8) -> Option<&String> {
        self.get_header(&HeaderName::Reference(index))
    }
}
