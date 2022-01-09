mod standard_header_name;
pub use standard_header_name::StandardHeaderName;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum HeaderName {
    Standard(StandardHeaderName),
    Reference(u8),
    Custom(String),
}

#[derive(Debug)]
pub struct HeaderNameParseError;

impl std::str::FromStr for HeaderName {
    type Err = HeaderNameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(header) = StandardHeaderName::from_str(s) {
            return Ok(HeaderName::Standard(header));
        }
        if s.get(..9) == Some("Reference") {
            if let Ok(index) = u8::from_str(&s[9..]) {
                return Ok(HeaderName::Reference(index));
            }
        }
        Ok(HeaderName::Custom(s.to_string()))
    }
}

impl HeaderName {
    pub fn from(s: &str) -> Self {
        HeaderName::from_str(s).unwrap()
    }
}

impl std::fmt::Display for HeaderName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeaderName::Standard(header) => f.write_str(header.into()),
            HeaderName::Reference(index) => write!(f, "Reference{}", index),
            HeaderName::Custom(name) => name.fmt(f),
        }
    }
}
