use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GenericResponse {
    pub status: u16,
    pub message: String,
}
