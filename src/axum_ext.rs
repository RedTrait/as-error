pub use axum::response::{IntoResponse, Response};

pub use crate::ServerError;

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        use axum::http::StatusCode;
        let res = match self {
            Self::HTTPParseError(info) => (StatusCode::BAD_REQUEST, info.to_string()),
        };
        res.into_response()
    }
}
