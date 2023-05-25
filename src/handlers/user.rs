use crate::{
    domain::user::{payload::{NewUserPayload, UpdateUserPayload}, PublicUser, UserRepository},
    repositories::error::RepositoryError,
};

pub type DynUserHandler = dyn UserHandler + Send + Sync;

pub struct UserHandlerImpl {
    pub user_repository: Box<dyn UserRepository + Send + Sync>,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait UserHandler {
    async fn create_user(&self, new_user: NewUserPayload) -> Result<PublicUser, RepositoryError>;

    async fn update_user_by_id(
        &self,
        id: i32,
        update_payload: UpdateUserPayload,
    ) -> Result<(), RepositoryError>;
    
    async fn get_user_by_nickname(
        &self,
        nickname: String,
    ) -> Result<Option<PublicUser>, RepositoryError>;
    async fn delete_user(&self, id: i32) -> Result<(), RepositoryError>;
}

#[async_trait::async_trait]
impl UserHandler for UserHandlerImpl {
    async fn create_user(&self, new_user: NewUserPayload) -> Result<PublicUser, RepositoryError> {
        let new_user = self.user_repository.create_user(new_user).await?;
        Ok(new_user)
    }

    async fn update_user_by_id(
        &self,
        id: i32,
        update_payload: UpdateUserPayload,
    ) -> Result<(), RepositoryError> {
        let user = self.user_repository.get_user_by_id(id).await?;
        if user.is_none() {
            return Err(RepositoryError::NotFound);
        }
        self.user_repository
            .update_user(id, update_payload)
            .await?;
        Ok(())
    }

    async fn get_user_by_nickname(
        &self,
        nickname: String,
    ) -> Result<Option<PublicUser>, RepositoryError> {
        let user = self.user_repository.get_user_by_nickname(nickname).await?;
        if user.is_none() {
            return Err(RepositoryError::NotFound);
        }
        Ok(user)
    }

    async fn delete_user(&self, id: i32) -> Result<(), RepositoryError> {
        let user = self.user_repository.get_user_by_id(id).await?;
        if user.is_none() {
            return Err(RepositoryError::NotFound);
        }
        Ok(())
    }
}
