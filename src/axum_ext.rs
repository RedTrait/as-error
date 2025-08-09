pub use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub use crate::ServerError;

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        use axum::http::StatusCode;
        let internal_server_error = StatusCode::INTERNAL_SERVER_ERROR;
        let bad_request = StatusCode::BAD_REQUEST;
        let res: Response = match self {
            Self::InvalidRequest(o) => (bad_request, o.to_string()).into_response(),
            Self::HTTPParseError(o) => (bad_request, o.to_string()).into_response(),
            #[cfg(feature = "sqlx_error")]
            Self::SQLXError(o) => {
                (StatusCode::INTERNAL_SERVER_ERROR, o.to_string()).into_response()
            }
            Self::MutexLockError => internal_server_error.into_response(),
        };
        res
    }
}
