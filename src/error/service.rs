pub use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("InvalidRequest: {0}")]
    InvalidRequest(String),

    #[error("PreconditionFailed: {0}")]
    PreconditionFailed(String),

    #[error("MutexLockError: {0}")]
    MutexLockError(String),

    #[error("JsonError: {0}")]
    JsonError(String),
}

impl From<serde_json::Error> for ServiceError {
    fn from(_err: serde_json::Error) -> Self {
        ServiceError::JsonError("".to_string())
    }
}
