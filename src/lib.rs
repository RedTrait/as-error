#[cfg(feature = "axum_ext")]
pub mod axum_ext;
#[cfg(feature = "file_error")]
pub mod file;
#[cfg(feature = "http_error")]
pub mod http;
#[cfg(feature = "service_error")]
pub mod service;
#[cfg(feature = "tokio_error")]
pub mod tokio;

pub use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[cfg(feature = "service_error")]
    #[error("ServiceError: {0}")]
    ServiceError(#[from] service::ServiceError),

    #[cfg(feature = "http_error")]
    #[error("HTTPError: {0}")]
    HTTPError(#[from] http::HTTPError),

    #[cfg(feature = "sqlx_error")]
    #[error("SQLXError: {0}")]
    SQLXError(#[from] sqlx::Error),

    #[cfg(feature = "file_error")]
    #[error("FileError: {0}")]
    FileError(#[from] file::FileError),

    #[cfg(feature = "redis_error")]
    #[error("RedisError: {0}")]
    RedisError(#[from] redis::RedisError),

    #[cfg(feature = "tokio_error")]
    #[error("TokioError: {0}")]
    TokioError(#[from] tokio::TokioError),
}

pub type ResultExt<T> = Result<T, ServerError>;
