use std::fmt::Display;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use argon2::password_hash::Error;
use strum::EnumMessage;

use crate::{repositories::error::RepositoryError, response::GenericResponse};

#[derive(Debug)]
pub enum ErrorType {
    NotFound,
    Conflict,
    InternalError,
    BadRequest,
}

#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub r#type: ErrorType,
}

impl From<RepositoryError> for AppError {
    fn from(error: RepositoryError) -> Self {
        match error {
            RepositoryError::NotFound => AppError {
                message: "Not found".to_string(),
                r#type: ErrorType::NotFound,
            },
            RepositoryError::Conflict(message) => AppError {
                message: message.get_message().unwrap().to_string(),
                r#type: ErrorType::Conflict,
            },
            RepositoryError::SqlxError(error) => AppError {
                message: format!("Internal error: {}", error),
                r#type: ErrorType::InternalError,
            },
            RepositoryError::HashingError(error) => match error {
                Error::Password => AppError {
                    message: format!("Invalid password"),
                    r#type: ErrorType::BadRequest,
                },
                _ => AppError {
                    message: format!("Internal error: {}", error),
                    r#type: ErrorType::InternalError,
                },
            }
        }
    }
}

impl AppError {
    pub fn bad_request(message: String) -> AppError {
        AppError {
            message,
            r#type: ErrorType::BadRequest,
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.r#type {
            ErrorType::NotFound => StatusCode::NOT_FOUND,
            ErrorType::Conflict => StatusCode::CONFLICT,
            ErrorType::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::BadRequest => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(GenericResponse {
            status: self.status_code().as_u16(),
            message: self.message.to_string(),
        })
    }
}
