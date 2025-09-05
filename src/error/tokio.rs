use tokio::task::JoinError;

#[derive(derive_more::Display, derive_more::Error, Debug)]
pub enum TokioError {
    #[display("JoinError: {}", _0)]
    TaskJoinError(JoinError),
}

impl From<JoinError> for TokioError {
    fn from(e: JoinError) -> Self {
        TokioError::TaskJoinError(e)
    }
}
