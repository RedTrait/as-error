use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::ServerError;

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let res: Response = match self {
            #[cfg(feature = "service_error")]
            Self::ServiceError(o) => {
                use crate::error::service::ServiceError;

                match o {
                    // TODO: https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/412 Incomplete implementation
                    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/If-Match
                    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/If-Unmodified-Since
                    // Consider adding header validation
                    ServiceError::PreconditionFailed(o) => {
                        (StatusCode::PRECONDITION_FAILED, o).into_response()
                    }
                    ServiceError::InvalidRequest(o) => (StatusCode::BAD_REQUEST, o).into_response(),
                    ServiceError::MutexLockError(o) | ServiceError::JsonError(o) => {
                        (StatusCode::INTERNAL_SERVER_ERROR, o).into_response()
                    }
                }
            }

            #[cfg(feature = "http_error")]
            Self::HTTPError(o) => (StatusCode::BAD_REQUEST, o.to_string()).into_response(),

            #[cfg(feature = "sqlx_error")]
            Self::SQLXError(o) => {
                (StatusCode::INTERNAL_SERVER_ERROR, o.to_string()).into_response()
            }

            #[cfg(feature = "redis_error")]
            Self::RedisError(o) => {
                (StatusCode::INTERNAL_SERVER_ERROR, o.to_string()).into_response()
            }

            #[cfg(feature = "file_error")]
            Self::FileError(o) => {
                use crate::error::file::FileError;
                match o {
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
                    FileError::FileFrozen => {
                        (StatusCode::FORBIDDEN, "File is frozen").into_response()
                    }
                    FileError::FileWriteFailed(_) => {
                        (StatusCode::INTERNAL_SERVER_ERROR, "File write failed").into_response()
                    }
                    FileError::FileReadFailed(_) => {
                        (StatusCode::INTERNAL_SERVER_ERROR, "File read failed").into_response()
                    }
                }
            }

            #[cfg(feature = "tokio_error")]
            Self::TokioError(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };
        res
    }
}
