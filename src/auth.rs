use std::env;

use chrono::Utc;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    exp: usize,
}

pub fn create_jwt(uuid: Uuid) -> Result<String, Error> {
    //TODO: instantiate in another place
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not set in .env file");

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uuid,
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(jwt_secret.as_bytes()))
}
