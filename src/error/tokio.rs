use derive_more::{Display, Error};
use tokio::task::JoinError;

#[derive(Display, Error, Debug)]
pub enum TokioError {
    #[display("JoinError: {}", _0)]
    TaskJoinError(JoinError),
}

impl From<JoinError> for TokioError {
    fn from(e: JoinError) -> Self {
        TokioError::TaskJoinError(e)
    }
}
