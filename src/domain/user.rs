use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct User {
    pub email: String,
    pub name: String,
    pub id: Uuid
}
