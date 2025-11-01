pub mod actix_ext;

pub mod error;
pub mod prelude;
pub use prelude::*;

#[allow(dead_code)]
pub mod const_define {
    // Service ERROR start with 00_XXXX
    pub(crate) const SERVICE_ERROR: &'static str = "00_0000";

    // SQLx ERROR start with 01_XXXX
    pub(crate) const SQLX_ERROR: &'static str = "01_0000";

    // File ERROR start with 02_XXXX
    pub const FILE_ERROR: &'static str = "02_0000";
    pub const FILE_CREATE_FAILED: &'static str = "02_0001";
    pub const FILE_DELETE_FAILED: &'static str = "02_0002";
    pub const FILE_NOT_FOUND: &'static str = "02_0003";
    pub const FILE_ALREADY_EXISTS: &'static str = "02_0004";
    pub const FILE_FROZEN: &'static str = "02_0005";
    pub const FILE_WRITE_FAILED: &'static str = "02_0006";
    pub const FILE_READ_FAILED: &'static str = "02_0007";

    // Redis ERROR start with 03_XXXX
    pub(crate) const REDIS_ERROR: &'static str = "03_0000";

    // Tokio ERROR start with 04_XXXX
    pub(crate) const TOKIO_ERROR: &'static str = "04_0000";
    pub(crate) const TOKIO_TASK_JOIN_ERROR: &'static str = "04_0001";

    pub(crate) const CLIENT_ERROR: &'static str = "05_0000";
}

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[cfg(feature = "service_error")]
    #[error("ServiceError: {0}")]
    ServiceError(#[from] ServiceError),

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

    #[cfg(feature = "client_error")]
    #[error("ConnectError: {0}")]
    ClientConnectError(#[from] awc::error::ConnectError),

    #[cfg(feature = "client_error")]
    #[error("FreezeRequestError: {0}")]
    ClientFreezeRequestError(#[from] awc::error::FreezeRequestError),

    #[cfg(feature = "client_error")]
    #[error("HttpError: {0}")]
    ClientHttpError(#[from] awc::error::HttpError),

    #[cfg(feature = "client_error")]
    #[error("JsonPayloadError: {0}")]
    ClientJsonPayloadError(#[from] awc::error::JsonPayloadError),

    #[cfg(feature = "client_error")]
    #[error("WsClientError: {0}")]
    ClientWsClientError(#[from] awc::error::WsClientError),

    #[cfg(feature = "client_error")]
    #[error("SendRequestError: {0}")]
    ClientSendRequestError(#[from] awc::error::SendRequestError),

    #[cfg(feature = "client_error")]
    #[error("WsHandshakeError: {0}")]
    ClientWsHandshakeError(#[from] awc::error::WsHandshakeError),

    #[cfg(feature = "client_error")]
    #[error("WsProtocolError: {0}")]
    ClientWsProtocolError(#[from] awc::error::WsProtocolError),
}

pub type ResultExt<T> = Result<T, ServerError>;
