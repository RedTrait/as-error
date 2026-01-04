use ractor::errors::{ActorErr, ActorProcessingErr, MessagingErr, RactorErr, SpawnErr};

#[derive(derive_more::Display, derive_more::Error, Debug)]
pub enum RactorError<MSG> {
    #[display("ActorError: {}", _0)]
    ActorError(ActorErr),
    #[display("MessagingError: {}", _0)]
    MessagingError(MessagingErr<MSG>),
    #[display("ActorProcessingError: {}", _0)]
    ActorProcessingError(ActorProcessingErr),
    #[display("SpawnError: {}", _0)]
    SpawnError(SpawnErr),
    #[display("RactorError: {}", _0)]
    RactorError(RactorErr<MSG>),
}
