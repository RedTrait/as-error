pub use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub use crate::ServerError;

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        use axum::http::StatusCode;

        #[cfg(feature = "file_error")]
        use crate::file::FileError;

        let res: Response = match self {
            // TODO: https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/412 Incomplete implementation
            // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/If-Match
            // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/If-Unmodified-Since
            // Consider adding header validation
            Self::PreconditionFailed(o) => (StatusCode::PRECONDITION_FAILED, o).into_response(),
            Self::InvalidRequest(o) => (StatusCode::BAD_REQUEST, o.to_string()).into_response(),
            Self::HTTPParseError(o) => (StatusCode::BAD_REQUEST, o.to_string()).into_response(),
            Self::ServiceError(o) => (StatusCode::INTERNAL_SERVER_ERROR, o).into_response(),

            #[cfg(feature = "sqlx_error")]
            Self::SQLXError(o) => {
                (StatusCode::INTERNAL_SERVER_ERROR, o.to_string()).into_response()
            }

            #[cfg(feature = "redis_error")]
            Self::RedisError(o) => {
                (StatusCode::INTERNAL_SERVER_ERROR, o.to_string()).into_response()
            }

            #[cfg(feature = "serde_json_error")]
            Self::SerdeJsonError(o) => {
                (StatusCode::INTERNAL_SERVER_ERROR, o.to_string()).into_response()
            }

            Self::MutexLockError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),

            #[cfg(feature = "file_error")]
            Self::FileError(o) => match o {
                FileError::FileCreateFailed(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "File create failed").into_response()
                }
                FileError::FileDeleteFailed(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "File delete failed").into_response()
                }
                FileError::FileNotFound => {
                    (StatusCode::NOT_FOUND, "File not found").into_response()
                }
                FileError::FileAlreadyExists => {
                    (StatusCode::CONFLICT, "File already exists").into_response()
                }
                FileError::FileFrozen => (StatusCode::FORBIDDEN, "File is frozen").into_response(),
                FileError::FileWriteFailed(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "File write failed").into_response()
                }
                FileError::FileReadFailed(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "File read failed").into_response()
                }
            },

            #[cfg(feature = "tokio_error")]
            Self::TokioError(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };
        res
    }
}
