use thiserror::Error;

#[cfg(feature = "axum_ext")]
pub mod axum_ext;

pub mod error;
pub mod prelude;
pub use prelude::*;

#[derive(Error, Debug)]
pub enum ServerError {
    #[cfg(feature = "service_error")]
    #[error("ServiceError: {0}")]
    ServiceError(#[from] ServiceError),

    #[cfg(feature = "http_error")]
    #[error("HTTPError: {0}")]
    HTTPError(#[from] HTTPError),

    #[cfg(feature = "sqlx_error")]
    #[error("SQLXError: {0}")]
    SQLXError(#[from] sqlx::Error),

    #[cfg(feature = "file_error")]
    #[error("FileError: {0}")]
    FileError(#[from] FileError),

    #[cfg(feature = "redis_error")]
    #[error("RedisError: {0}")]
    RedisError(#[from] redis::RedisError),

    #[cfg(feature = "tokio_error")]
    #[error("TokioError: {0}")]
    TokioError(#[from] TokioError),
}

pub type ResultExt<T> = Result<T, ServerError>;
