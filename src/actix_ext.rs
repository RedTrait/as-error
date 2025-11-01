use crate::ServerError;
use crate::const_define::*;
use actix_web::{HttpResponse, ResponseError, body::BoxBody, http::StatusCode};

impl ResponseError for ServerError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            #[cfg(feature = "service_error")]
            Self::ServiceError(o) => {
                use crate::error::service::ServiceError;
                match o {
                    // TODO: https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/412 Incomplete implementation
                    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/If-Match
                    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/If-Unmodified-Since
                    // Consider adding header validation
                    ServiceError::PreconditionFailed(_, _)
                    | ServiceError::StaticPreconditionFailed(_, _) => {
                        StatusCode::PRECONDITION_FAILED
                    }

                    ServiceError::InvalidRequest(_, _)
                    | ServiceError::StaticInvalidRequest(_, _) => StatusCode::BAD_REQUEST,

                    ServiceError::MutexLockError(_, _)
                    | ServiceError::StaticMutexLockError(_, _)
                    | ServiceError::JsonError(_, _)
                    | ServiceError::StaticJsonError(_, _)
                    | ServiceError::InternalServiceError(_, _)
                    | ServiceError::StaticInternalServiceError(_, _) => {
                        StatusCode::INTERNAL_SERVER_ERROR
                    }

                    ServiceError::RetryError(_, _) | ServiceError::StaticRetryError(_, _) => {
                        StatusCode::SERVICE_UNAVAILABLE
                    }
                }
            }

            #[cfg(feature = "sqlx_error")]
            Self::SQLXError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            #[cfg(feature = "redis_error")]
            Self::RedisError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            #[cfg(feature = "file_error")]
            Self::FileError(o) => {
                use crate::error::file::FileError;
                match o {
                    FileError::FileCreateFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    FileError::FileDeleteFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    FileError::FileNotFound => StatusCode::NOT_FOUND,
                    FileError::FileAlreadyExists => StatusCode::CONFLICT,
                    FileError::FileFrozen => StatusCode::FORBIDDEN,
                    FileError::FileWriteFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    FileError::FileReadFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
                }
            }

            #[cfg(feature = "tokio_error")]
            Self::TokioError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            #[cfg(feature = "client_error")]
            Self::ClientConnectError(_)
            | Self::ClientFreezeRequestError(_)
            | Self::ClientHttpError(_)
            | Self::ClientJsonPayloadError(_)
            | Self::ClientWsClientError(_)
            | Self::ClientSendRequestError(_)
            | Self::ClientWsHandshakeError(_)
            | Self::ClientWsProtocolError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            #[cfg(feature = "service_error")]
            Self::ServiceError(o) => {
                use crate::error::service::ServiceError;
                match o {
                    // TODO: https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/412 Incomplete implementation
                    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/If-Match
                    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/If-Unmodified-Since
                    // Consider adding header validation
                    ServiceError::PreconditionFailed(_err, info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::PRECONDITION_FAILED).body(info.clone())
                    }
                    ServiceError::StaticPreconditionFailed(_err, info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::PRECONDITION_FAILED).body(*info)
                    }
                    ServiceError::InvalidRequest(_err, info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::BAD_REQUEST).body(info.clone())
                    }
                    ServiceError::StaticInvalidRequest(_err, info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::BAD_REQUEST).body(*info)
                    }
                    ServiceError::MutexLockError(_err, _info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(SERVICE_ERROR)
                    }
                    ServiceError::StaticMutexLockError(_err, _info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(SERVICE_ERROR)
                    }
                    ServiceError::JsonError(_err, info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(info.clone())
                    }
                    ServiceError::StaticJsonError(_err, info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(*info)
                    }
                    ServiceError::RetryError(_err, info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).body(info.clone())
                    }
                    ServiceError::StaticRetryError(_err, info) => {
                        HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).body(*info)
                    }
                    ServiceError::InternalServiceError(_err, info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(info.clone())
                    }
                    ServiceError::StaticInternalServiceError(_, info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(*info)
                    }
                }
            }

            #[cfg(feature = "sqlx_error")]
            Self::SQLXError(_) => {
                // TODO: write to log
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(SQLX_ERROR)
            }

            #[cfg(feature = "redis_error")]
            Self::RedisError(_) => {
                // TODO: write to log
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(REDIS_ERROR)
            }

            #[cfg(feature = "file_error")]
            Self::FileError(o) => {
                use crate::error::file::FileError;
                match o {
                    FileError::FileCreateFailed(_) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(FILE_CREATE_FAILED)
                    }
                    FileError::FileDeleteFailed(_) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(FILE_DELETE_FAILED)
                    }
                    FileError::FileNotFound => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::NOT_FOUND).body(FILE_NOT_FOUND)
                    }
                    FileError::FileAlreadyExists => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::CONFLICT).body(FILE_ALREADY_EXISTS)
                    }
                    FileError::FileFrozen => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::FORBIDDEN).body(FILE_FROZEN)
                    }
                    FileError::FileWriteFailed(_) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(FILE_WRITE_FAILED)
                    }
                    FileError::FileReadFailed(_) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(FILE_READ_FAILED)
                    }
                }
            }

            #[cfg(feature = "tokio_error")]
            Self::TokioError(o) => match o {
                crate::TokioError::TaskJoinError(_join_error) => {
                    // TODO: write to log
                    HttpResponse::build(StatusCode::PRECONDITION_FAILED).body(TOKIO_TASK_JOIN_ERROR)
                }
            },

            #[cfg(feature = "client_error")]
            Self::ClientConnectError(_)
            | Self::ClientFreezeRequestError(_)
            | Self::ClientHttpError(_)
            | Self::ClientJsonPayloadError(_)
            | Self::ClientWsClientError(_)
            | Self::ClientSendRequestError(_)
            | Self::ClientWsHandshakeError(_)
            | Self::ClientWsProtocolError(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(CLIENT_ERROR)
            }
        }
    }
}
