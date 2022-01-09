#[derive(
    Debug,
    PartialEq,
    Eq,
    Hash,
    strum_macros::EnumString,
    strum_macros::Display,
    strum_macros::IntoStaticStr,
)]
pub enum Method {
    GET,
    NOTIFY,
    #[strum(serialize = "GET Version")]
    GETVersion,
    #[strum(serialize = "GET Sentence")]
    GETSentence,
    #[strum(serialize = "GET Word")]
    GETWord,
    #[strum(serialize = "GET Status")]
    GETStatus,
    TEACH,
    #[strum(serialize = "GET String")]
    GETString,
    #[strum(serialize = "NOTIFY OwnerGhostName")]
    NOTIFYOwnerGhostName,
    #[strum(serialize = "NOTIFY OtherGhostName")]
    NOTIFYOtherGhostName,
    #[strum(serialize = "TRANSLATE Sentence")]
    TRANSLATESentence,
    TRANSLATE,
}
