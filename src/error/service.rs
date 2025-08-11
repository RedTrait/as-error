use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("InvalidRequest: {0}")]
    InvalidRequest(String),

    #[error("StaticInvalidRequest: {0}")]
    StaticInvalidRequest(&'static str),

    #[error("PreconditionFailed: {0}")]
    PreconditionFailed(String),

    #[error("StaticPreconditionFailed: {0}")]
    StaticPreconditionFailed(&'static str),

    #[error("MutexLockError: {0}")]
    MutexLockError(String),

    #[error("StaticMutexLockError: {0}")]
    StaticMutexLockError(&'static str),

    #[error("JsonError: {0}")]
    JsonError(String),

    #[error("StaticJsonError: {0}")]
    StaticJsonError(&'static str),

    #[error("RetryError")]
    RetryError,
}

impl From<serde_json::Error> for ServiceError {
    fn from(_err: serde_json::Error) -> Self {
        // add error detail in log file, but not return detail to send req user
        ServiceError::StaticJsonError("Failed to parse JSON")
    }
}
