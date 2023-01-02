use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("No user with email `{0}`")]
    NoUserByEmail(String),
    #[error("User with emai `{0}` already exists")]
    UserWithEmailAlreadyExists(String),
}
