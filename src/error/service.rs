use std::error::Error;

use thiserror::Error;

/// first argument use in local dev
/// second argument use to response user
#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("InvalidRequest: {0}")]
    InvalidRequest(Box<dyn Error + Send + Sync + 'static>, String),

    #[error("StaticInvalidRequest: {0}")]
    StaticInvalidRequest(Box<dyn Error + Send + Sync + 'static>, &'static str),

    #[error("PreconditionFailed: {0}")]
    PreconditionFailed(Box<dyn Error + Send + Sync + 'static>, String),

    #[error("StaticPreconditionFailed: {0}")]
    StaticPreconditionFailed(Box<dyn Error + Send + Sync + 'static>, &'static str),

    #[error("MutexLockError: {0}")]
    MutexLockError(Box<dyn Error + Send + Sync + 'static>, String),

    #[error("StaticMutexLockError: {0}")]
    StaticMutexLockError(Box<dyn Error + Send + Sync + 'static>, &'static str),

    #[error("JsonError: {0}")]
    JsonError(Box<dyn Error + Send + Sync + 'static>, String),

    #[error("StaticJsonError: {0}")]
    StaticJsonError(Box<dyn Error + Send + Sync + 'static>, &'static str),

    #[error("RetryError: {0}")]
    RetryError(Box<dyn Error + Send + Sync>, String),

    #[error("StaticRetryError: {0}")]
    StaticRetryError(Box<dyn Error + Send + Sync + 'static>, &'static str),

    #[error("InternalServiceError: {0}")]
    InternalServiceError(Box<dyn Error + Send + Sync>, String),

    #[error("StaticInternalServiceError: {0}")]
    StaticInternalServiceError(Box<dyn Error + Send + Sync + 'static>, &'static str),
}

impl From<serde_json::Error> for ServiceError {
    fn from(err: serde_json::Error) -> Self {
        // add error detail in log file, but not return detail to send req user
        ServiceError::StaticJsonError(Box::new(err), "Failed to parse JSON")
    }
}

#[cfg(test)]
pub mod tests {
    use crate::*;

    #[derive(thiserror::Error, Debug)]
    pub enum TestThisError {
        #[error("TestThisError")]
        TestError,
    }

    #[derive(derive_more::Display, derive_more::Error, Debug)]
    pub enum TestderiveMoreError {
        #[display("TestderiveMoreError")]
        TestError,
    }

    #[test]
    fn invalid_request() {
        let error = invalid_request!(TestThisError::TestError, "H").to_string();
        assert_eq!("ServiceError: InvalidRequest: TestThisError", error);
        let error = invalid_request!(TestderiveMoreError::TestError, "H").to_string();
        assert_eq!("ServiceError: InvalidRequest: TestderiveMoreError", error);
        let error = invalid_request!(TestThisError::TestError, "H".to_owned()).to_string();
        assert_eq!("ServiceError: InvalidRequest: TestThisError", error);
    }

    #[test]
    fn static_invalid_request() {
        let error = static_invalid_request!(TestThisError::TestError, "H").to_string();
        assert_eq!("ServiceError: StaticInvalidRequest: TestThisError", error);
        let error = static_invalid_request!(TestderiveMoreError::TestError, "H").to_string();
        assert_eq!(
            "ServiceError: StaticInvalidRequest: TestderiveMoreError",
            error
        );
    }

    #[test]
    fn precondition_failed() {
        let error = precondition_failed!(TestThisError::TestError, "H").to_string();
        assert_eq!("ServiceError: PreconditionFailed: TestThisError", error);
        let error = precondition_failed!(TestderiveMoreError::TestError, "H").to_string();
        assert_eq!(
            "ServiceError: PreconditionFailed: TestderiveMoreError",
            error
        );
    }

    #[test]
    fn static_precondition_failed() {
        let error = static_precondition_failed!(TestThisError::TestError, "H").to_string();
        assert_eq!(
            "ServiceError: StaticPreconditionFailed: TestThisError",
            error
        );
        let error = static_precondition_failed!(TestderiveMoreError::TestError, "H").to_string();
        assert_eq!(
            "ServiceError: StaticPreconditionFailed: TestderiveMoreError",
            error
        );
    }

    #[test]
    fn mutex_lock_error() {
        let error = mutex_lock_error!(TestThisError::TestError, "H").to_string();
        assert_eq!("ServiceError: MutexLockError: TestThisError", error);
        let error = mutex_lock_error!(TestderiveMoreError::TestError, "H").to_string();
        assert_eq!("ServiceError: MutexLockError: TestderiveMoreError", error);
    }

    #[test]
    fn static_mutex_lock_error() {
        let error = static_mutex_lock_error!(TestThisError::TestError, "H").to_string();
        assert_eq!("ServiceError: StaticMutexLockError: TestThisError", error);
        let error = static_mutex_lock_error!(TestderiveMoreError::TestError, "H").to_string();
        assert_eq!(
            "ServiceError: StaticMutexLockError: TestderiveMoreError",
            error
        );
    }

    #[test]
    fn json_error() {
        let error = json_error!(TestThisError::TestError, "H").to_string();
        assert_eq!("ServiceError: JsonError: TestThisError", error);
        let error = json_error!(TestderiveMoreError::TestError, "H").to_string();
        assert_eq!("ServiceError: JsonError: TestderiveMoreError", error);
    }

    #[test]
    fn static_json_error() {
        let error = static_json_error!(TestThisError::TestError, "H").to_string();
        assert_eq!("ServiceError: StaticJsonError: TestThisError", error);
        let error = static_json_error!(TestderiveMoreError::TestError, "H").to_string();
        assert_eq!("ServiceError: StaticJsonError: TestderiveMoreError", error);
    }

    #[test]
    fn retry_error() {
        let error = retry_error!(TestThisError::TestError, "H").to_string();
        assert_eq!("ServiceError: RetryError: TestThisError", error);
        let error = retry_error!(TestderiveMoreError::TestError, "H").to_string();
        assert_eq!("ServiceError: RetryError: TestderiveMoreError", error);
    }

    #[test]
    fn static_retry_error() {
        let error = static_retry_error!(TestThisError::TestError, "H").to_string();
        assert_eq!("ServiceError: StaticRetryError: TestThisError", error);
        let error = static_retry_error!(TestderiveMoreError::TestError, "H").to_string();
        assert_eq!("ServiceError: StaticRetryError: TestderiveMoreError", error);
    }

    #[test]
    fn internal_service_error() {
        let error = internal_service_error!(TestThisError::TestError, "H").to_string();
        assert_eq!("ServiceError: InternalServiceError: TestThisError", error);
        let error = internal_service_error!(TestderiveMoreError::TestError, "H").to_string();
        assert_eq!(
            "ServiceError: InternalServiceError: TestderiveMoreError",
            error
        );
    }

    #[test]
    fn static_internal_service_error() {
        let error = static_internal_service_error!(TestThisError::TestError, "H").to_string();
        assert_eq!(
            "ServiceError: StaticInternalServiceError: TestThisError",
            error
        );
        let error = static_internal_service_error!(TestderiveMoreError::TestError, "H").to_string();
        assert_eq!(
            "ServiceError: StaticInternalServiceError: TestderiveMoreError",
            error
        );
    }
}
