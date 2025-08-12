use as_error::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignUpError {
    #[error("normal avatar not create")]
    NormalAvatarNotCreate,
}

fn main() {
    let e = SignUpError::NormalAvatarNotCreate;
    let quick_error = static_precondition_failed!(e, "This is a static precondition failed error");
    println!("Quick error: {}", quick_error);
    let e = SignUpError::NormalAvatarNotCreate;
    let info = String::from("This is a string error");
    let error = precondition_failed!(e, info);
    println!("Error: {}", error);
}
