use std::fmt;
use sqlx::Error as SqlxError;
use argon2::password_hash::Error as Argon2Error;
use strum::EnumMessage;
use strum_macros;

#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    Conflict(ErrorMessage),
    SqlxError(SqlxError),
    HashingError(Argon2Error)
}

impl std::error::Error for RepositoryError {}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::NotFound => write!(f, "Not found"),
            RepositoryError::Conflict(error_message) => {
                let message = error_message.get_message().unwrap().to_string();
                write!(f, "{message}")}
            RepositoryError::SqlxError(error) => write!(f, "Internal error: {}", error),
            RepositoryError::HashingError(error) => write!(f, "Internal error: {}", error),
        }
    }
}

impl From<SqlxError> for RepositoryError {
    fn from(error: SqlxError) -> Self {
        RepositoryError::SqlxError(error)
    }
}

impl From<Argon2Error> for RepositoryError {
    fn from(error: Argon2Error) -> Self {
        RepositoryError::HashingError(error)
    }
}

#[derive(strum_macros::EnumMessage, Debug)]
#[allow(dead_code)]
pub enum ErrorMessage {
    #[strum(message = "This nickname is already in use")]
    ExistingNickame,
    #[strum(message = "This email is already in use")]
    ExistingEmail,
}
