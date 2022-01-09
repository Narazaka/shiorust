use crate::message::parts::HeaderName;
use crate::message::Request;
use crate::message::Response;

pub trait SetHeader<T> {
    fn set_header(&mut self, name: T, value: String) -> Option<String>;
}

impl SetHeader<&str> for Request {
    fn set_header(&mut self, name: &str, value: String) -> Option<String> {
        self.headers.insert(HeaderName::from(name), value)
    }
}

impl SetHeader<HeaderName> for Request {
    fn set_header(&mut self, name: HeaderName, value: String) -> Option<String> {
        self.headers.insert(name, value)
    }
}

impl SetHeader<&str> for Response {
    fn set_header(&mut self, name: &str, value: String) -> Option<String> {
        self.headers.insert(HeaderName::from(name), value)
    }
}

impl SetHeader<HeaderName> for Response {
    fn set_header(&mut self, name: HeaderName, value: String) -> Option<String> {
        self.headers.insert(name, value)
    }
}
