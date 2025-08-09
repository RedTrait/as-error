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
}

pub type ResultExt<T> = Result<T, ServerError>;
