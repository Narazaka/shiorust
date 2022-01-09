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
