use std::io;

use derive_more::{Display, Error};

/// A set of errors that about file operations.
#[derive(Debug, Display, Error)]
#[non_exhaustive]
pub enum FileError {
    #[display("file create failed: {}", _0)]
    FileCreateFailed(io::Error),

    #[display("file delete failed: {}", _0)]
    FileDeleteFailed(io::Error),

    #[display("file not found")]
    FileNotFound,

    #[display("file already exists")]
    FileAlreadyExists,

    #[display("file frozen")]
    FileFrozen,

    #[display("file write failed: {}", _0)]
    FileWriteFailed(io::Error),

    #[display("file read failed: {}", _0)]
    FileReadFailed(io::Error),
}
