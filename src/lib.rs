pub mod http_parse;

pub use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("HTTP Parse: {0}")]
    HTTPParseError(#[from] http_parse::ParseError),
}

#[cfg(feature = "axum")]
pub use axum::response::{IntoResponse, Response};

#[cfg(feature = "axum")]
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        use axum::http::StatusCode;
        let res = match self {
            Self::HTTPParseError(info) => (StatusCode::BAD_REQUEST, info.to_string()),
        };
        res.into_response()
    }
}
