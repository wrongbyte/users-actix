use sqlx::PgPool;

use crate::domain::user::{
    payload::{NewUserPayload, UpdateUserPayload},
    PublicUser, UserRepository,
};

use super::error::RepositoryError;

pub struct SqlUserRepository {
    pub pool: PgPool,
}

#[async_trait::async_trait]
impl UserRepository for SqlUserRepository {
    async fn create_user(&self, user: NewUserPayload) -> Result<PublicUser, RepositoryError> {
        let row = sqlx::query_as::<_, PublicUser>(
            "INSERT INTO users (nickname, email, password, bio) VALUES ($1, $2, $3, $4)
            RETURNING name, nickname, email, bio, creation_time::TIMESTAMPTZ",
        )
        .bind(user.nickname)
        .bind(user.email)
        .bind(user.password)
        .bind(user.bio)
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    async fn update_user(&self, id: i32, user: UpdateUserPayload) -> Result<(), RepositoryError> {
        todo!()
    }

    async fn get_user_by_nickname(
        &self,
        nickname: String,
    ) -> Result<Option<PublicUser>, RepositoryError> {
        let row = sqlx::query_as::<_, PublicUser>(
            "SELECT name, nickname, email, bio, creation_time::TIMESTAMPTZ FROM users WHERE nickname = $1",
        )
        .bind(nickname)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn get_user_by_id(&self, id: i32) -> Result<Option<PublicUser>, RepositoryError> {
        todo!()
    }

    async fn get_user_by_email(
        &self,
        email: String,
    ) -> Result<Option<PublicUser>, RepositoryError> {
        todo!()
    }

    async fn delete_user(&self, id: i32) -> Result<(), RepositoryError> {
        todo!()
    }
}
