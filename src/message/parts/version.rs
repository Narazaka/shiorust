#[derive(
    Debug,
    PartialEq,
    Eq,
    Hash,
    strum_macros::EnumString,
    strum_macros::Display,
    strum_macros::IntoStaticStr,
)]
pub enum Version {
    #[strum(serialize = "2.0")]
    V20,
    #[strum(serialize = "2.2")]
    V22,
    #[strum(serialize = "2.3")]
    V23,
    #[strum(serialize = "2.4")]
    V24,
    #[strum(serialize = "2.5")]
    V25,
    #[strum(serialize = "2.6")]
    V26,
    #[strum(serialize = "3.0")]
    V30,
}
