pub use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub use crate::ServerError;

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        use axum::http::StatusCode;
        let internal_server_error = StatusCode::INTERNAL_SERVER_ERROR;
        let res: Response = match self {
            Self::HTTPParseError(o) => (StatusCode::BAD_REQUEST, o.to_string()).into_response(),
            #[cfg(feature = "sqlx_error")]
            Self::SQLXError(o) => {
                (StatusCode::INTERNAL_SERVER_ERROR, o.to_string()).into_response()
            }
            Self::MutexLockError => internal_server_error.into_response(),
        };
        res
    }
}
