use async_trait::async_trait;
use http_problem::Result;

use crate::domain::user::User;

pub struct NewUserPayload {
    pub name: Option<String>,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
}

pub struct UpdateUserPayload {
    pub name: Option<String>,
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
}


#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserHandler {
    async fn create_user(&self, new_user: NewUserPayload) -> Result<()>;
    async fn update_user(&self, id: i64, update_payload: UpdateUserPayload) -> Result<()>;
    async fn get_user_by_id(&self, id: i64) -> Result<User>;
    async fn get_user_by_nickname(&self, nickname: String) -> Result<User>;
    async fn delete_user(&self, id: i64) -> Result<()>;
}
