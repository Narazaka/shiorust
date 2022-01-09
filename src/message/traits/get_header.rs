use crate::message::parts::HeaderName;
use crate::message::Request;
use crate::message::Response;

pub trait GetHeader<T> {
    fn get_header(&self, name: T) -> Option<&String>;
}

impl GetHeader<&str> for Request {
    fn get_header(&self, name: &str) -> Option<&String> {
        self.headers.get(&HeaderName::from(name))
    }
}

impl GetHeader<&HeaderName> for Request {
    fn get_header(&self, name: &HeaderName) -> Option<&String> {
        self.headers.get(name)
    }
}

impl GetHeader<&str> for Response {
    fn get_header(&self, name: &str) -> Option<&String> {
        self.headers.get(&HeaderName::from(name))
    }
}

impl GetHeader<&HeaderName> for Response {
    fn get_header(&self, name: &HeaderName) -> Option<&String> {
        self.headers.get(name)
    }
}
