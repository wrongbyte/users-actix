use std::fmt;
use tokio_postgres::Error;

#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    Conflict,
    InternalError(Error),
}

impl std::error::Error for RepositoryError {}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::NotFound => write!(f, "Not found"),
            RepositoryError::Conflict => write!(f, "Conflict"),
            RepositoryError::InternalError(error) => write!(f, "Internal error: {}", error),
        }
    }
}

impl From<Error> for RepositoryError {
    fn from(error: Error) -> Self {
        RepositoryError::InternalError(error)
    }
}