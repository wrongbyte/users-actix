use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    repositories::error::RepositoryError,
    utils::{serialize_dt, serialize_dt_option},
};

use self::payload::{LoginUserPayload, NewUserPayload, UpdateUserPayload};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub nickname: String,
    pub email: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(serialize_with = "serialize_dt")]
    pub creation_time: DateTime<Utc>,
    #[serde(serialize_with = "serialize_dt_option")]
    pub update_time: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublicUser {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub nickname: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(serialize_with = "serialize_dt")]
    pub creation_time: DateTime<Utc>,
}

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create_user(&self, user: NewUserPayload) -> Result<PublicUser, RepositoryError>;
    async fn update_user(&self, id: Uuid, user: UpdateUserPayload) -> Result<(), RepositoryError>;
    async fn get_user_by_nickname(
        &self,
        nickname: String,
    ) -> Result<Option<PublicUser>, RepositoryError>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<PublicUser>, RepositoryError>;
    async fn get_user_by_email(&self, email: String)
        -> Result<Option<PublicUser>, RepositoryError>;
    async fn get_user_by_login(
        &self,
        login_payload: LoginUserPayload,
    ) -> Result<PublicUser, RepositoryError>;
    async fn delete_user(&self, id: Uuid) -> Result<(), RepositoryError>;
}

pub mod payload {
    use lazy_static::lazy_static;
    use regex::Regex;
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    lazy_static! {
        static ref NICKNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    }

    #[derive(sqlx::FromRow, Serialize, Deserialize, Validate)]
    pub struct NewUserPayload {
        #[validate(length(min = 3, max = 15))]
        pub name: Option<String>,
        #[validate(regex = "NICKNAME_REGEX")]
        pub nickname: String,
        #[validate(email)]
        pub email: String,
        #[validate(length(min = 8))]
        pub password: String,
        #[validate(length(max = 250))]
        pub bio: Option<String>,
    }

    #[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Validate)]
    pub struct UpdateUserPayload {
        #[validate(length(min = 3, max = 15))]
        pub name: Option<String>,
        #[validate(regex = "NICKNAME_REGEX")]
        pub nickname: Option<String>,
        #[validate(length(max = 250))]
        pub bio: Option<String>,
    }

    #[derive(Serialize, Deserialize, Validate)]
    pub struct LoginUserPayload {
        #[validate(email)]
        pub email: String,
        #[validate(length(min = 8))]
        pub password: String,
    }
}

pub mod validation {
    use std::collections::HashMap;

    use validator::ValidationError;

    pub fn format_error_msg(hash_map: HashMap<&'static str, &Vec<ValidationError>>) -> String {
        let mut error_fields_vec: Vec<String> = vec![];
        for (key, _) in hash_map.iter() {
            error_fields_vec.push(key.to_string())
        }
        let fields = error_fields_vec
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");
        format!("The following fields contain validation errors: {}", fields) //TODO: return better error messages because this thing is currently despicable
    }
}

pub mod password {
    use argon2::{password_hash::Error, PasswordHash, PasswordVerifier};

    use super::*;
    pub fn hash_password(original: String) -> Result<String, Error> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(original.as_bytes(), &salt)?
            .to_string();
        Ok(password_hash)
    }

    pub fn verify_passwords(input_password: String, db_password: String) -> Result<(), Error> {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&db_password).unwrap();
        argon2.verify_password(input_password.as_bytes(), &parsed_hash)?;
        Ok(())
    }
}
