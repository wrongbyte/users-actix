use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    repositories::error::RepositoryError,
    utils::{serialize_dt, serialize_dt_option},
};

use self::payload::{NewUserPayload, UpdateUserPayload};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublicUser {
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
    async fn update_user(&self, id: i32, user: UpdateUserPayload) -> Result<(), RepositoryError>;
    async fn get_user_by_nickname(
        &self,
        nickname: String,
    ) -> Result<Option<PublicUser>, RepositoryError>;
    async fn get_user_by_id(&self, id: i32) -> Result<Option<PublicUser>, RepositoryError>;
    async fn get_user_by_email(&self, email: String)
        -> Result<Option<PublicUser>, RepositoryError>;
    async fn delete_user(&self, id: i32) -> Result<(), RepositoryError>;
}

pub mod payload {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct NewUserPayload {
        pub name: Option<String>,
        pub nickname: String,
        pub email: String,
        pub password: String,
        pub bio: Option<String>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct UpdateUserPayload {
        pub name: Option<String>,
        pub nickname: Option<String>,
        pub bio: Option<String>,
    }
}
