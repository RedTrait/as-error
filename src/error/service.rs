use std::error::Error;

use thiserror::Error;

use crate::ServerError;

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

    #[error("InternalServiceError: {0}")]
    InternalServiceError(Box<dyn Error + Send + Sync>, String),

    #[error("StaticInternalServiceError: {0}")]
    StaticInternalServiceError(Box<dyn Error + Send + Sync + 'static>, &'static str),
}

impl From<serde_json::Error> for ServiceError {
    fn from(_err: serde_json::Error) -> Self {
        // add error detail in log file, but not return detail to send req user
        ServiceError::StaticJsonError("Failed to parse JSON")
    }
}

impl ServerError {
    pub fn static_internal_service_error<E>(err: E, msg: &'static str) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        ServerError::ServiceError(ServiceError::StaticInternalServiceError(Box::new(err), msg))
    }

    pub fn internal_service_error<E, S>(err: E, msg: S) -> Self
    where
        E: Error + Send + Sync + 'static,
        S: Into<String>,
    {
        ServerError::ServiceError(ServiceError::InternalServiceError(
            Box::new(err),
            msg.into(),
        ))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[derive(thiserror::Error, Debug)]
    pub enum TestThisError {
        #[error("could not generate image")]
        TestError,
    }

    #[derive(derive_more::Display, derive_more::Error, Debug)]
    pub enum TestderiveMoreError {
        #[display("TestderiveMoreError")]
        TestError,
    }

    #[test]
    fn internal_service_error() {
        ServerError::internal_service_error(TestThisError::TestError, "H".to_owned());
        ServerError::internal_service_error(TestderiveMoreError::TestError, "H".to_owned());
    }

    #[test]
    fn static_internal_service_error() {
        ServerError::static_internal_service_error(TestThisError::TestError, "H");
        ServerError::static_internal_service_error(TestderiveMoreError::TestError, "H");
    }
}
