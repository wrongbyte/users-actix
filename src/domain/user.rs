use chrono::{
    DateTime, Utc,
};
use http_problem::Result;
use serde::{Deserialize, Serialize};

use crate::utils::{serialize_dt, serialize_dt_option};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
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

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create_user(&self, user: User) -> Result<()>;
    async fn update_user(&self, user: User) -> Result<()>;
    async fn get_user_by_id(&self, id: i64) -> Result<User>;
    async fn get_user_by_nickname(&self, nickname: String) -> Result<User>;
    async fn get_user_by_email(&self, email: String) -> Result<User>;
    async fn delete_user(&self, id: i64) -> Result<()>;
}
