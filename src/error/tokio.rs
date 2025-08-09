use derive_more::{Display, Error};
use tokio::task::JoinError;

#[derive(Display, Error, Debug)]
pub enum TokioError {
    #[display("JoinError: {}", _0)]
    TaskJoinError(JoinError),
}
