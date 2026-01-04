pub mod actix_ext;

pub mod error;
pub mod prelude;
pub use prelude::*;

// TODO 这里所有的编码得整理一下
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

    pub(crate) const AWC_ERROR: &'static str = "05_0000";
    pub(crate) const REQWEST_ERROR: &'static str = "06_0000";

    pub(crate) const CHRONO_PARSE_ERROR: &'static str = "07_0001";
    pub(crate) const TIME_ZONE_ERROR: &'static str = "07_0002";
    pub(crate) const WEEK_ERROR: &'static str = "07_0003";

    pub(crate) const DECIMAL_ERROR: &'static str = "08_0000";

    pub(crate) const FLATBUFFER_ERROR: &'static str = "09_0000";

    pub(crate) const HTTP_RESPONSE_ERROR: &'static str = "10_0000";

    pub(crate) const STRING_FROM_UTF8_ERROR: &'static str = "10_0001";

    pub(crate) const SERDE_JSON_ERROR: &'static str = "11_0001";

    pub(crate) const RACTOR_ERROR: &'static str = "12_0000";
}

#[derive(thiserror::Error, Debug)]
#[allow(dead_code)]
pub enum AsError<MSG = ()> {
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

    #[cfg(feature = "chrono_error")]
    #[error("ChronoParseError: {0}")]
    ChronoParseError(#[from] chrono::ParseError),

    #[cfg(feature = "chrono_error")]
    #[error("TimeZoneError")]
    TimeZoneError,

    #[cfg(feature = "chrono_error")]
    #[error("WeekError")]
    WeekError,

    #[cfg(feature = "rust_decimal_error")]
    #[error("DecimalError: {0}")]
    DecimalError(#[from] rust_decimal::Error),

    #[cfg(feature = "awc_error")]
    #[error("AwcConnectError: {0}")]
    AwcConnectError(#[from] awc::error::ConnectError),

    #[cfg(feature = "awc_error")]
    #[error("AwcFreezeRequestError: {0}")]
    AwcFreezeRequestError(#[from] awc::error::FreezeRequestError),

    #[cfg(feature = "awc_error")]
    #[error("AwcHttpError: {0}")]
    AwcHttpError(#[from] awc::error::HttpError),

    #[cfg(feature = "awc_error")]
    #[error("AwcJsonPayloadError: {0}")]
    AwcJsonPayloadError(#[from] awc::error::JsonPayloadError),

    #[cfg(feature = "awc_error")]
    #[error("AwcPayloadError: {0}")]
    AwcPayloadError(#[from] awc::error::PayloadError),

    #[cfg(feature = "awc_error")]
    #[error("AwcWsClientError: {0}")]
    AwcWsClientError(#[from] awc::error::WsClientError),

    #[cfg(feature = "awc_error")]
    #[error("AwcSendRequestError: {0}")]
    AwcSendRequestError(#[from] awc::error::SendRequestError),

    #[cfg(feature = "awc_error")]
    #[error("AwcWsHandshakeError: {0}")]
    AwcWsHandshakeError(#[from] awc::error::WsHandshakeError),

    #[cfg(feature = "awc_error")]
    #[error("AwcWsProtocolError: {0}")]
    AwcWsProtocolError(#[from] awc::error::WsProtocolError),

    #[cfg(feature = "reqwest_error")]
    #[error("ReqwestError: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[cfg(feature = "flatbuffer_error")]
    #[error("InvalidFlatbufferError")]
    InvalidFlatbufferError,

    #[cfg(feature = "http_response_error")]
    #[error("HttpResponseNotOK: {0}")]
    HttpResponseNotOK(#[from] HttpResponseNotOK),

    #[cfg(feature = "string_error")]
    #[error("StringError: {0}")]
    StringError(#[from] StringError),

    #[cfg(feature = "serde_error")]
    #[error("SerdeError: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[cfg(feature = "ractor_error")]
    #[error("RactorError: {0}")]
    RactorError(#[from] RactorError<MSG>),

    #[cfg(feature = "ractor_error")]
    #[error("RactorDummError")]
    RactorDummError,
}
#[cfg(not(feature = "ractor_error"))]
pub type ResultExt<T> = Result<T, AsError<()>>;

#[cfg(feature = "ractor_error")]
pub type ResultExt<T, MSG = ()> = Result<T, AsError<MSG>>;

#[cfg(feature = "ractor_error")]
pub trait ResultExtCast<T> {
    fn cast<MSG>(self) -> ResultExt<T, MSG>;
}

#[cfg(feature = "ractor_error")]
impl<T> ResultExtCast<T> for Result<T, AsError<()>> {
    fn cast<MSG>(self) -> ResultExt<T, MSG> {
        match self {
            Ok(it) => Ok(it),
            Err(e) => match e {
                AsError::ServiceError(error) => Err(AsError::<MSG>::ServiceError(error)),
                AsError::SQLXError(error) => Err(AsError::<MSG>::SQLXError(error)),
                AsError::FileError(error) => Err(AsError::<MSG>::FileError(error)),
                AsError::RedisError(error) => Err(AsError::<MSG>::RedisError(error)),
                AsError::TokioError(error) => Err(AsError::<MSG>::TokioError(error)),
                AsError::ChronoParseError(error) => Err(AsError::<MSG>::ChronoParseError(error)),
                AsError::TimeZoneError => Err(AsError::<MSG>::TimeZoneError),
                AsError::WeekError => Err(AsError::<MSG>::WeekError),
                AsError::DecimalError(error) => Err(AsError::<MSG>::DecimalError(error)),
                AsError::AwcConnectError(error) => Err(AsError::<MSG>::AwcConnectError(error)),
                AsError::AwcFreezeRequestError(error) => {
                    Err(AsError::<MSG>::AwcFreezeRequestError(error))
                }
                AsError::AwcHttpError(error) => Err(AsError::<MSG>::AwcHttpError(error)),
                AsError::AwcJsonPayloadError(error) => {
                    Err(AsError::<MSG>::AwcJsonPayloadError(error))
                }
                AsError::AwcPayloadError(error) => Err(AsError::<MSG>::AwcPayloadError(error)),
                AsError::AwcWsClientError(error) => Err(AsError::<MSG>::AwcWsClientError(error)),
                AsError::AwcSendRequestError(error) => {
                    Err(AsError::<MSG>::AwcSendRequestError(error))
                }
                AsError::AwcWsHandshakeError(error) => {
                    Err(AsError::<MSG>::AwcWsHandshakeError(error))
                }
                AsError::AwcWsProtocolError(error) => {
                    Err(AsError::<MSG>::AwcWsProtocolError(error))
                }
                AsError::ReqwestError(error) => Err(AsError::<MSG>::ReqwestError(error)),
                AsError::InvalidFlatbufferError => Err(AsError::<MSG>::InvalidFlatbufferError),
                AsError::HttpResponseNotOK(error) => Err(AsError::<MSG>::HttpResponseNotOK(error)),
                AsError::StringError(error) => Err(AsError::<MSG>::StringError(error)),
                AsError::SerdeJsonError(error) => Err(AsError::<MSG>::SerdeJsonError(error)),
                AsError::RactorError(_) => Err(AsError::<MSG>::RactorDummError),
                AsError::RactorDummError => Err(AsError::<MSG>::RactorDummError),
            },
        }
    }
}
