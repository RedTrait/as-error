use std::string::FromUtf8Error;

#[derive(derive_more::Display, derive_more::Error, Debug)]
pub enum StringError {
    FromUtf8Error(FromUtf8Error),
}
