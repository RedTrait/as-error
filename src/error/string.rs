use std::string::FromUtf8Error;

#[derive(derive_more::Display, derive_more::Error, Debug)]
pub enum StringError {
    FromUtf8Error(FromUtf8Error),
}

impl From<FromUtf8Error> for StringError {
    fn from(e: FromUtf8Error) -> Self {
        StringError::FromUtf8Error(e)
    }
}
