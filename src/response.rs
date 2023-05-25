use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GenericResponse {
    pub status: u16,
    pub message: String,
}

impl GenericResponse {
    pub fn not_found() -> GenericResponse {
        GenericResponse {
            status: StatusCode::NOT_FOUND.as_u16(),
            message: String::from("Not found"),
        }
    }

    pub fn bad_request(message: String) -> GenericResponse {
        GenericResponse {
            status: StatusCode::BAD_REQUEST.as_u16(),
            message: String::from(format!("Bad request: {message}")),
        }
    }
}
