use async_trait::async_trait;
use chrono::{DateTime, Utc};
use http_problem::Result;
use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Serialize)]
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
    pub creation_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create_user(&self, user: User) -> Result<()>;
    async fn update_user(&self, user: User) -> Result<()>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<User>;
    async fn get_user_by_nickname(&self, nickname: String) -> Result<User>;
    async fn get_user_by_email(&self, email: String) -> Result<User>;
    async fn delete_user(&self, id: Uuid) -> Result<()>;
}
