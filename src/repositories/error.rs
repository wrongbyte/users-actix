use std::fmt;
use sqlx::Error;
use strum::EnumMessage;
use strum_macros;

#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    Conflict(ErrorMessage),
    InternalError(Error),
}

impl std::error::Error for RepositoryError {}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::NotFound => write!(f, "Not found"),
            RepositoryError::Conflict(error_message) => {
                let message = error_message.get_message().unwrap().to_string();
                write!(f, "{message}")}
            RepositoryError::InternalError(error) => write!(f, "Internal error: {}", error),
        }
    }
}

impl From<Error> for RepositoryError {
    fn from(error: Error) -> Self {
        RepositoryError::InternalError(error)
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
