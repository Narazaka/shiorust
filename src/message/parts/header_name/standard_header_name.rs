use super::super::HeaderName;
use super::super::Headers;
use paste::paste;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headers() {
        let mut h = Headers::new();
        h.insert_by_header_name(Sender, "foo".to_string());
        assert_eq!(h.get_by_header_name(&Sender), Some(&"foo".to_string()));
        assert_eq!(h.get_sender(), Some(&"foo".to_string()));
    }
}

macro_rules! standard_header_names {
    (
        $(
            $(#[$docs:meta])*
            $konst:ident,
        )+
    ) => {
        /// SHIORI/2.x, 3.0 standard header names
        #[derive(
            Debug,
            PartialEq,
            Eq,
            Hash,
            strum_macros::EnumString,
            strum_macros::Display,
            strum_macros::IntoStaticStr,
        )]
        pub enum StandardHeaderName {
            $(
                $(#[$docs])*
                $konst,
            )+
        }

        paste! {
            $(
                #[allow(non_upper_case_globals)]
                $(#[$docs])*
                pub const $konst: HeaderName = HeaderName::Standard(StandardHeaderName::$konst);
            )+

            impl Headers {
                $(
                    $(#[$docs])*
                    pub fn [<get_ $konst:snake:lower>](&self) -> Option<&String> {
                        self.get_by_header_name(&HeaderName::Standard(StandardHeaderName::$konst))
                    }

                    $(#[$docs])*
                    pub fn [<set_ $konst:snake:lower>](&mut self, value: String) -> Option<String> {
                        self.insert_by_header_name(HeaderName::Standard(StandardHeaderName::$konst), value)
                    }
                )+
            }
        }
    }
}

standard_header_names! {
    /// Sender header
    Sender,
    /// Charset header
    Charset,
    /// SecurityLevel header (SHIORI/2.2,2.6,3.x)
    SecurityLevel,
    /// ID header (SHIORI/2.5,3.x)
    ID,
    /// Event header (SHIORI/2.2)
    Event,
    /// Type header (GET Word SHIORI/2.0)
    Type,
    /// Status header (SHIORI/3.0 SSP extended)
    Status,
    /// Ghost header (NOTIFY OwnerGhostName SHIORI/2.0,2.3)
    Ghost,
    /// Sentence header (SHIORI/2.0,2.3b)
    Sentence,
    /// To header (SHIORI/2.3b)
    To,
    /// Age header (SHIORI/2.3b)
    Age,
    /// Surface header (SHIORI/2.3b)
    Surface,
    /// Word header (TEACH SHIORI/2.4)
    Word,
}
