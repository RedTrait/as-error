//! Currently using the definition in crate actix_web
use std::{io, str::Utf8Error, string::FromUtf8Error};

use derive_more::{Display, Error};
use http::uri::InvalidUri;

#[derive(Debug, Display, Error)]
pub enum HTTPError {
    #[display("http stream parse error: {}", _0)]
    ParseError(ParseError),
    #[display("http stream content type error: {}", _0)]
    ContentTypeError(ContentTypeError),
}

/// A set of errors that can occur during parsing HTTP streams.
#[derive(Debug, Display, Error)]
#[non_exhaustive]
pub enum ParseError {
    /// An invalid `Method`, such as `GE.T`.
    #[display("invalid method specified")]
    Method,

    /// An invalid `Uri`, such as `exam ple.domain`.
    #[display("URI error: {}", _0)]
    Uri(InvalidUri),

    /// An invalid `HttpVersion`, such as `HTP/1.1`
    #[display("invalid HTTP version specified")]
    Version,

    /// An invalid `Header`.
    #[display("invalid Header provided")]
    Header,

    /// A message head is too large to be reasonable.
    #[display("message head is too large")]
    TooLarge,

    /// A message reached EOF, but is not complete.
    #[display("message is incomplete")]
    Incomplete,

    /// An invalid `Status`, such as `1337 ELITE`.
    #[display("invalid status provided")]
    Status,

    /// A timeout occurred waiting for an IO event.
    #[allow(dead_code)]
    #[display("timeout")]
    Timeout,

    /// An I/O error that occurred while trying to read or write to a network stream.
    #[display("I/O error: {}", _0)]
    Io(io::Error),

    /// Parsing a field as string failed.
    #[display("UTF-8 error: {}", _0)]
    Utf8(Utf8Error),
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> ParseError {
        ParseError::Io(err)
    }
}

impl From<InvalidUri> for ParseError {
    fn from(err: InvalidUri) -> ParseError {
        ParseError::Uri(err)
    }
}

impl From<Utf8Error> for ParseError {
    fn from(err: Utf8Error) -> ParseError {
        ParseError::Utf8(err)
    }
}

impl From<FromUtf8Error> for ParseError {
    fn from(err: FromUtf8Error) -> ParseError {
        ParseError::Utf8(err.utf8_error())
    }
}

impl From<httparse::Error> for ParseError {
    fn from(err: httparse::Error) -> ParseError {
        match err {
            httparse::Error::HeaderName
            | httparse::Error::HeaderValue
            | httparse::Error::NewLine
            | httparse::Error::Token => ParseError::Header,
            httparse::Error::Status => ParseError::Status,
            httparse::Error::TooManyHeaders => ParseError::TooLarge,
            httparse::Error::Version => ParseError::Version,
        }
    }
}

/// A set of error that can occur during parsing content type.
#[derive(Debug, Display, Error)]
#[cfg_attr(test, derive(PartialEq, Eq))]
#[non_exhaustive]
pub enum ContentTypeError {
    /// Can not parse content type.
    #[display("could not parse content type")]
    ParseError,

    /// Unknown content encoding.
    #[display("unknown content encoding")]
    UnknownEncoding,
}
