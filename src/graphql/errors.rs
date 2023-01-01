use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("No user with email `{0}`")]
    NoUserByEmail(String),
}
