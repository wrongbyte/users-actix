use std::sync::Arc;

use http_problem::Result;

use crate::{
    domain::user::{User, UserRepository},
    routes::user::{NewUserPayload, UpdateUserPayload},
};

pub type DynUserHandler = dyn UserHandler + Send + Sync;

pub struct UserHandlerImpl {
    pub user_repository: Box<dyn UserRepository + Send + Sync>,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait UserHandler {
    async fn create_user(&self, new_user: NewUserPayload) -> Result<()>;
    async fn update_user(&self, id: i64, update_payload: UpdateUserPayload) -> Result<()>;
    async fn get_user_by_id(&self, id: i64) -> Result<User>;
    async fn get_user_by_nickname(&self, nickname: String) -> Result<User>;
    async fn delete_user(&self, id: i64) -> Result<()>;
}

#[async_trait::async_trait]
impl UserHandler for UserHandlerImpl {
    async fn create_user(&self, new_user: NewUserPayload) -> Result<()> {
        self.user_repository.create_user(new_user).await?;
        Ok(())
    }

    async fn update_user(&self, id: i64, update_payload: UpdateUserPayload) -> Result<()> {
        todo!()
    }

    async fn get_user_by_id(&self, id: i64) -> Result<User> {
        todo!()
    }

    async fn get_user_by_nickname(&self, nickname: String) -> Result<User> {
        todo!()
    }

    async fn delete_user(&self, id: i64) -> Result<()> {
        todo!()
    }
}
