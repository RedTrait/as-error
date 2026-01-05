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

impl<MSG> From<ActorErr> for RactorError<MSG> {
    fn from(e: ActorErr) -> Self {
        RactorError::ActorError(e)
    }
}

impl<MSG> From<MessagingErr<MSG>> for RactorError<MSG> {
    fn from(e: MessagingErr<MSG>) -> Self {
        RactorError::MessagingError(e)
    }
}

impl<MSG> From<ActorProcessingErr> for RactorError<MSG> {
    fn from(e: ActorProcessingErr) -> Self {
        RactorError::ActorProcessingError(e)
    }
}

impl<MSG> From<SpawnErr> for RactorError<MSG> {
    fn from(e: SpawnErr) -> Self {
        RactorError::SpawnError(e)
    }
}

impl<MSG> From<RactorErr<MSG>> for RactorError<MSG> {
    fn from(e: RactorErr<MSG>) -> Self {
        RactorError::RactorError(e)
    }
}
