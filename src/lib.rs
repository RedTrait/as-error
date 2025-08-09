#[cfg(feature = "axum")]
pub mod axum_ext;

pub mod http_parse;

pub use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("PreconditionFailed: {0}")]
    PreconditionFailed(String),

    #[error("Invalid Request: {0}")]
    InvalidRequest(String),

    #[error("HTTP Parse: {0}")]
    HTTPParseError(#[from] http_parse::ParseError),

    #[error("mutex lock error")]
    MutexLockError,

    #[cfg(feature = "sqlx_error")]
    #[error("SQLXError info: {0}")]
    SQLXError(#[from] sqlx::Error),
}

pub type ResultExt<T> = Result<T, ServerError>;
