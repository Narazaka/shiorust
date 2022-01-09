#[derive(
    Debug,
    PartialEq,
    Eq,
    Hash,
    strum_macros::EnumString,
    strum_macros::Display,
    strum_macros::IntoStaticStr,
    strum_macros::EnumMessage,
)]
pub enum Status {
    #[strum(serialize = "200 OK", message = "OK")]
    OK = 200,
    #[strum(serialize = "204 No Content", message = "No Content")]
    NoContent = 204,
    #[strum(serialize = "310 Communicate", message = "Communicate")]
    Communicate = 310,
    #[strum(serialize = "311 Not Enough", message = "Not Enough")]
    NotEnough = 311,
    #[strum(serialize = "312 Advice", message = "Advice")]
    Advice = 312,
    #[strum(serialize = "400 Bad Request", message = "Bad Request")]
    BadRequest = 400,
    #[strum(
        serialize = "500 Internal Server Error",
        message = "Internal Server Error"
    )]
    InternalServerError = 500,
}
