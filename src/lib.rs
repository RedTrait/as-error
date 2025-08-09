#[cfg(feature = "axum")]
pub mod axum_ext;

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

    #[error("MutexLockError")]
    MutexLockError,

    #[cfg(feature = "sqlx_error")]
    #[error("SQLXError: {0}")]
    SQLXError(#[from] sqlx::Error),

    #[cfg(feature = "file_error")]
    #[error("FileNotFound")]
    FileNotFound,

    #[cfg(feature = "file_error")]
    #[error("FileAlreadyExists")]
    FileAlreadyExists,

    #[cfg(feature = "file_error")]
    #[error("FileFrozen")]
    FileFrozen,

    #[cfg(feature = "file_error")]
    #[error("FileWriteFailed: {0}")]
    FileWriteFailed(String),

    #[cfg(feature = "file_error")]
    #[error("FileReadFailed")]
    FileReadFailed,

    #[cfg(feature = "file_error")]
    #[error("FileDeleteFailed: {0}")]
    FileDeleteFailed(String),

    #[cfg(feature = "redis_error")]
    #[error("RedisError: {0}")]
    RedisError(#[from] redis::RedisError),
}

pub type ResultExt<T> = Result<T, ServerError>;
