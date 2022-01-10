pub mod standard_header_name;
pub use standard_header_name::StandardHeaderName;
use std::str::FromStr;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_header_name() {
        assert_eq!(HeaderName::from("Charset").to_string(), "Charset");
        assert_eq!(
            HeaderName::from("Charset"),
            HeaderName::Standard(StandardHeaderName::Charset)
        );
        assert_eq!(HeaderName::from("Reference11").to_string(), "Reference11");
        assert_eq!(HeaderName::from("Reference11"), HeaderName::Reference(11));
        assert_eq!(HeaderName::from("X-Foo").to_string(), "X-Foo");
        assert_eq!(
            HeaderName::from("X-Foo"),
            HeaderName::Custom("X-Foo".to_string())
        );
    }
}

/// SHIORI header name
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum HeaderName {
    /// standard headers
    Standard(StandardHeaderName),
    /// Reference* header
    Reference(u16),
    /// other headers
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
            if let Ok(index) = u16::from_str(&s[9..]) {
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
