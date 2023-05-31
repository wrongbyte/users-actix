use uuid::Uuid;

use crate::repositories::error::ErrorMessage::{ExistingNickame, ExistingEmail};
use crate::{
    domain::user::{
        payload::{NewUserPayload, UpdateUserPayload},
        PublicUser, UserRepository,
    },
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
        id: Uuid,
        update_payload: UpdateUserPayload,
    ) -> Result<(), RepositoryError>;

    async fn get_user_by_nickname(
        &self,
        nickname: String,
    ) -> Result<Option<PublicUser>, RepositoryError>;
    async fn delete_user(&self, id: Uuid) -> Result<(), RepositoryError>;

    async fn get_user_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<PublicUser>, RepositoryError>;
}

#[async_trait::async_trait]
impl UserHandler for UserHandlerImpl {
    async fn create_user(&self, new_user: NewUserPayload) -> Result<PublicUser, RepositoryError> {
        let user_with_nickname = self.user_repository.get_user_by_nickname(new_user.nickname.clone()).await?;
        if user_with_nickname.is_some() {
            return Err(RepositoryError::Conflict(ExistingNickame));
        }

        let user_with_email = self.user_repository.get_user_by_email(new_user.email.clone()).await?;
        if user_with_email.is_some() {
            return Err(RepositoryError::Conflict(ExistingEmail));
        }

        let new_user = self.user_repository.create_user(new_user).await?;
        Ok(new_user)
    }

    async fn update_user_by_id(
        &self,
        id: Uuid,
        update_payload: UpdateUserPayload,
    ) -> Result<(), RepositoryError> {
        let user = self.user_repository.get_user_by_id(id).await?;
        if user.is_none() {
            return Err(RepositoryError::NotFound);
        }
        if let Some(nickname) = update_payload.nickname.clone() {
            let user_with_nickname = self.user_repository.get_user_by_nickname(nickname).await?;
            if user_with_nickname.is_some() {
                return Err(RepositoryError::Conflict(ExistingNickame));
            }
        }
        self.user_repository.update_user(id, update_payload).await?;
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

    async fn get_user_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<PublicUser>, RepositoryError> {
        let user = self.user_repository.get_user_by_id(id).await?;
        if user.is_none() {
            return Err(RepositoryError::NotFound);
        }
        Ok(user)
    }

    async fn delete_user(&self, id: Uuid) -> Result<(), RepositoryError> {
        let user = self.user_repository.get_user_by_id(id).await?;
        if user.is_none() {
            return Err(RepositoryError::NotFound);
        }
        self.user_repository.delete_user(id).await?;
        Ok(())
    }
}
