use awc::http::StatusCode;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display("http response status: {status} body: {body}")]
pub struct HttpResponseNotOK {
    pub status: StatusCode,
    pub body: String,
}
