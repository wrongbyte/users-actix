use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: u64,
    pub message: String,
}

impl GenericResponse {
    pub fn not_found() -> GenericResponse {
        GenericResponse {
            status: 404,
            message: String::from("Not found"),
        }
    }

    pub fn bad_request(message: String) -> GenericResponse {
        GenericResponse {
            status: 400,
            message: String::from(format!("Bad request: {message}")),
        }
    }
}
