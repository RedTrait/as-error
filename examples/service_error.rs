use as_error::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignUpError {
    #[error("normal avatar not create")]
    NormalAvatarNotCreate,
}

fn main() {
    let e = SignUpError::NormalAvatarNotCreate;
    let _ = ServerError::static_internal_service_error(e, "Failed to create normal avatar");
}
