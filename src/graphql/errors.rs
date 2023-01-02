use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("No user with email `{0}`")]
    NoUserByEmail(String),
    #[error("User with emai `{0}` already exists")]
    UserWithEmailAlreadyExists(String),
}

#[derive(Error, Debug)]
pub enum AuthorizationError {
    #[error(
        "Required permission is {} but user has permission {}",
        required,
        permission
    )]
    InsufficientPermission {
        required: String,
        permission: String,
    },
    #[error("Unable to decode JWT: {0}")]
    DecodingError(String),
}
