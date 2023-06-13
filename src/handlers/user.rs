use uuid::Uuid;

use crate::domain::user::payload::LoginUserPayload;
use crate::repositories::error::ErrorMessage::{ExistingEmail, ExistingNickame};
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

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<PublicUser>, RepositoryError>;

    async fn delete_user(&self, id: Uuid) -> Result<(), RepositoryError>;

    async fn get_user_by_login(
        &self,
        login_payload: LoginUserPayload,
    ) -> Result<Uuid, RepositoryError>;
}

#[async_trait::async_trait]
impl UserHandler for UserHandlerImpl {
    #[tracing::instrument(skip(self))]
    async fn create_user(&self, new_user: NewUserPayload) -> Result<PublicUser, RepositoryError> {
        let user_with_nickname = self
            .user_repository
            .get_user_by_nickname(new_user.nickname.clone())
            .await?;
        if user_with_nickname.is_some() {
            return Err(RepositoryError::Conflict(ExistingNickame));
        }

        let user_with_email = self
            .user_repository
            .get_user_by_email(new_user.email.clone())
            .await?;
        if user_with_email.is_some() {
            return Err(RepositoryError::Conflict(ExistingEmail));
        }

        let new_user = self.user_repository.create_user(new_user).await?;
        Ok(new_user)
    }

    #[tracing::instrument(skip(self))]
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

    #[tracing::instrument(skip(self))]
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

    #[tracing::instrument(skip(self))]
    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<PublicUser>, RepositoryError> {
        let user = self.user_repository.get_user_by_id(id).await?;
        if user.is_none() {
            return Err(RepositoryError::NotFound);
        }
        Ok(user)
    }

    #[tracing::instrument(skip(self))]
    async fn delete_user(&self, id: Uuid) -> Result<(), RepositoryError> {
        let user = self.user_repository.get_user_by_id(id).await?;
        if user.is_none() {
            return Err(RepositoryError::NotFound);
        }
        self.user_repository.delete_user(id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn get_user_by_login(
        &self,
        login_payload: LoginUserPayload,
    ) -> Result<Uuid, RepositoryError> {
        let user = self
            .user_repository
            .get_user_by_email(login_payload.email.clone())
            .await?;

        if user.is_none() {
            return Err(RepositoryError::NotFound);
        }

        let user = self
            .user_repository
            .get_user_by_login(login_payload)
            .await?;

        Ok(user.id)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::user::{mocks::*, MockUserRepository};

    #[tokio::test]
    async fn creates_user_correctly() {
        let new_user_payload = factori::create!(NewUserPayload);
        let user = factori::create!(PublicUser);

        let mut repo = MockUserRepository::new();

        repo.expect_get_user_by_nickname().returning(|_| Ok(None));

        repo.expect_get_user_by_email().returning(|_| Ok(None));

        repo.expect_create_user().return_once(|_| Ok(user));

        let handler = UserHandlerImpl {
            user_repository: Box::new(repo),
        };

        handler
            .create_user(new_user_payload)
            .await
            .expect("Failed to create a new user");
    }
}
