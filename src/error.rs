use axum::{http::StatusCode, response::IntoResponse};

/// when login failed, we have responsibility of creating a error
/// and give client back a `Response` type to make it clear and safe
/// from exposing raw `Result` type from our source code

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
}

/// this is a bolilerplate code for custom `Error` type
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

impl IntoResponse for Error {
    /// `Response` type can wrap things up to clients
    fn into_response(self) -> axum::response::Response {
        // debug code
        println!("@@@@@ {:<12} - {self:?}", "INTO_RES");

        // just return tuple of predefined error code and string slice
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
