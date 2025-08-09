pub use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub use crate::ServerError;

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        use axum::http::StatusCode;
        let internal_server_error = StatusCode::INTERNAL_SERVER_ERROR;
        let bad_request = StatusCode::BAD_REQUEST;
        let res: Response = match self {
            // TODO: https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/412 Incomplete implementation
            // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/If-Match
            // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/If-Unmodified-Since
            // Consider adding header validation
            Self::PreconditionFailed(o) => (StatusCode::PRECONDITION_FAILED, o).into_response(),
            Self::InvalidRequest(o) => (bad_request, o.to_string()).into_response(),
            Self::HTTPParseError(o) => (bad_request, o.to_string()).into_response(),
            #[cfg(feature = "sqlx_error")]
            Self::SQLXError(o) => {
                (StatusCode::INTERNAL_SERVER_ERROR, o.to_string()).into_response()
            }
            Self::MutexLockError => internal_server_error.into_response(),
        };
        res
    }
}
