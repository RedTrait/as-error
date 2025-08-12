#[macro_export]
macro_rules! internal_request {
    ($err:expr, $info:expr) => {
        ServerError::ServiceError(ServiceError::InvalidRequest(Box::new($err), $info.into()))
    };
}

#[macro_export]
macro_rules! static_invalid_request {
    ($err:expr, $info:expr) => {
        ServerError::ServiceError(ServiceError::StaticInvalidRequest(Box::new($err), $info))
    };
}

#[macro_export]
macro_rules! precondition_failed {
    ($err:expr, $info:expr) => {
        ServerError::ServiceError(ServiceError::PreconditionFailed(
            Box::new($err),
            $info.into(),
        ))
    };
}

#[macro_export]
macro_rules! static_precondition_failed {
    ($err:expr, $info:expr) => {
        ServerError::ServiceError(ServiceError::StaticPreconditionFailed(
            Box::new($err),
            $info,
        ))
    };
}

#[macro_export]
macro_rules! mutex_lock_error {
    ($err:expr, $info:expr) => {
        ServerError::ServiceError(ServiceError::MutexLockError(Box::new($err), $info.into()))
    };
}

#[macro_export]
macro_rules! static_mutex_lock_error {
    ($err:expr, $info:expr) => {
        ServerError::ServiceError(ServiceError::StaticMutexLockError(Box::new($err), $info))
    };
}

#[macro_export]
macro_rules! json_error {
    ($err:expr, $info:expr) => {
        ServerError::ServiceError(ServiceError::JsonError(Box::new($err), $info.into()))
    };
}

#[macro_export]
macro_rules! static_json_error {
    ($err:expr, $info:expr) => {
        ServerError::ServiceError(ServiceError::StaticJsonError(Box::new($err), $info))
    };
}

#[macro_export]
macro_rules! retry_error {
    ($err:expr, $info:expr) => {
        ServerError::ServiceError(ServiceError::RetryError(Box::new($err), $info.into()))
    };
}

#[macro_export]
macro_rules! static_retry_error {
    ($err:expr, $info:expr) => {
        ServerError::ServiceError(ServiceError::StaticRetryError(Box::new($err), $info))
    };
}

#[macro_export]
macro_rules! internal_service_error {
    ($err:expr, $info:expr) => {
        ServerError::ServiceError(ServiceError::InternalServiceError(
            Box::new($err),
            $info.into(),
        ))
    };
}

#[macro_export]
macro_rules! static_internal_service_error {
    ($err:expr, $info:expr) => {
        ServerError::ServiceError(ServiceError::StaticInternalServiceError(
            Box::new($err),
            $info,
        ))
    };
}
