use derive_more::{Display, Error};
use reqwest::StatusCode;

#[derive(Debug, Display, Error)]
#[display("http response status: {status} body: {body}")]
pub struct HttpResponseNotOK {
    pub status: StatusCode,
    pub body: String,
}
