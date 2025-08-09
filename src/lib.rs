#[cfg(feature = "axum")]
pub mod axum_ext;
#[cfg(feature = "file_error")]
pub mod file;
#[cfg(feature = "tokio_error")]
pub mod tokio;

pub mod http_parse;

pub use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("PreconditionFailed: {0}")]
    PreconditionFailed(String),

    #[error("InvalidRequest: {0}")]
    InvalidRequest(String),

    #[error("HTTPParseError: {0}")]
    HTTPParseError(#[from] http_parse::ParseError),

    #[error("ServiceError: {0}")]
    ServiceError(String),

    #[error("MutexLockError")]
    MutexLockError,

    #[cfg(feature = "sqlx_error")]
    #[error("SQLXError: {0}")]
    SQLXError(#[from] sqlx::Error),

    #[cfg(feature = "file_error")]
    #[error("FileError: {0}")]
    FileError(#[from] file::FileError),

    #[cfg(feature = "redis_error")]
    #[error("RedisError: {0}")]
    RedisError(#[from] redis::RedisError),

    #[cfg(feature = "serde_json_error")]
    #[error("SerdeJsonError: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[cfg(feature = "tokio_error")]
    #[error("TokioError: {0}")]
    TokioError(#[from] tokio::TokioError),
}

pub type ResultExt<T> = Result<T, ServerError>;
