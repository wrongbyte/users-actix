use std::env;

use actix_web::http::header::HeaderValue;
use chrono::Utc;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    exp: usize,
}

pub fn create_jwt(uuid: Uuid) -> Result<String, Error> {
    //TODO: instantiate in another place
    let jwt_secret =
        env::var("JWT_ENCODING_SECRET").expect("JWT_ENCODING_SECRET not set in .env file");

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(300))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uuid,
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
}

pub fn get_id_auth_header(header: &HeaderValue) -> Result<Uuid, AppError> {
    let header_str = header
        .to_str()
        .map_err(|_| AppError::bad_request("Invalid header".to_string()))?;

    //TODO: instantiate in another place
    let jwt_secret =
        env::var("JWT_ENCODING_SECRET").expect("JWT_ENCODING_SECRET not set in .env file");

    let token = decode::<Claims>(
        header_str,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|_| AppError::bad_request("Invalid token".to_string()))?;

    Ok(token.claims.sub)
}
