pub use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub use crate::ServerError;

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        use axum::http::StatusCode;

        let res: Response = match self {
            // TODO: https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/412 Incomplete implementation
            // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/If-Match
            // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/If-Unmodified-Since
            // Consider adding header validation
            Self::PreconditionFailed(o) => (StatusCode::PRECONDITION_FAILED, o).into_response(),
            Self::InvalidRequest(o) => (StatusCode::BAD_REQUEST, o.to_string()).into_response(),
            Self::HTTPParseError(o) => (StatusCode::BAD_REQUEST, o.to_string()).into_response(),
            #[cfg(feature = "sqlx_error")]
            Self::SQLXError(o) => {
                (StatusCode::INTERNAL_SERVER_ERROR, o.to_string()).into_response()
            }
            Self::MutexLockError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            #[cfg(feature = "file_error")]
            Self::FileNotFound => (StatusCode::NOT_FOUND, "File not found").into_response(),
            #[cfg(feature = "file_error")]
            Self::FileAlreadyExists => {
                (StatusCode::CONFLICT, "File already exists").into_response()
            }
            #[cfg(feature = "file_error")]
            Self::FileFrozen => (StatusCode::FORBIDDEN, "File is frozen").into_response(),
            #[cfg(feature = "file_error")]
            Self::FileWriteFailed(o) => (StatusCode::INTERNAL_SERVER_ERROR, o).into_response(),
            #[cfg(feature = "file_error")]
            Self::FileReadFailed => {
                (StatusCode::INTERNAL_SERVER_ERROR, "File read failed").into_response()
            }
            #[cfg(feature = "file_error")]
            Self::FileDeleteFailed(o) => (StatusCode::INTERNAL_SERVER_ERROR, o).into_response(),
        };
        res
    }
}
