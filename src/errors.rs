use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("No user with email `{0}`")]
    NoUserByEmail(String),
    #[error("User with email `{0}` already exists")]
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
    #[error("Unable to encode JWT: {0}")]
    EncodingError(String),
    #[error("Token has expired")]
    TokenExpired,
}

#[derive(Error, Debug)]
pub enum TimeError {
    #[error("Unable to compute present time")]
    NowError,
    #[error("Unable to determine {0} seconds from now")]
    CalculationError(u64),
}
