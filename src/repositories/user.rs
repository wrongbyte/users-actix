use crate::domain::user::{
    payload::{NewUserPayload, UpdateUserPayload},
    PublicUser, UserRepository, password::hash_password,
};
use chrono::Utc;
use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use super::error::RepositoryError;

pub struct SqlUserRepository {
    pub pool: PgPool,
}

#[async_trait::async_trait]
impl UserRepository for SqlUserRepository {
    async fn create_user(&self, user: NewUserPayload) -> Result<PublicUser, RepositoryError> {
        let uuid = Uuid::new_v4();
        let hashed_password = hash_password(user.password)?;
        let row = sqlx::query_as::<_, PublicUser>(
            "INSERT INTO users (id, name, nickname, email, password, bio) VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, nickname, email, bio, creation_time::TIMESTAMPTZ",
        )
        .bind(uuid)
        .bind(user.name)
        .bind(user.nickname)
        .bind(user.email)
        .bind(hashed_password)
        .bind(user.bio)
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    async fn update_user(&self, id: Uuid, user: UpdateUserPayload) -> Result<(), RepositoryError> {
        let mut query = get_update_query(user, id);
        query.build().execute(&self.pool).await?;

        Ok(())
    }

    async fn get_user_by_nickname(
        &self,
        nickname: String,
    ) -> Result<Option<PublicUser>, RepositoryError> {
        let row = sqlx::query_as::<_, PublicUser>(
            "SELECT id, name, nickname, email, bio, creation_time::TIMESTAMPTZ FROM users WHERE nickname = $1",
        )
        .bind(nickname)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<PublicUser>, RepositoryError> {
        let row = sqlx::query_as::<_, PublicUser>(
            "SELECT id, name, nickname, email, bio, creation_time::TIMESTAMPTZ FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn get_user_by_email(
        &self,
        email: String,
    ) -> Result<Option<PublicUser>, RepositoryError> {
        let row = sqlx::query_as::<_, PublicUser>(
            "SELECT id, name, nickname, email, bio, creation_time::TIMESTAMPTZ FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn delete_user(&self, id: Uuid) -> Result<(), RepositoryError> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

fn get_update_query(user: UpdateUserPayload, id: Uuid) -> QueryBuilder<'static, Postgres> {
    let mut query_builder = QueryBuilder::new("UPDATE users SET");

    let mut separated = query_builder.separated(", ");

    if let Some(name) = user.name {
        separated.push(" name = ");
        separated.push_bind_unseparated(name);
    };
    if let Some(nickname) = user.nickname {
        separated.push(" nickname = ");
        separated.push_bind_unseparated(nickname);
    };
    if let Some(bio) = user.bio {
        separated.push(" bio = ");
        separated.push_bind_unseparated(bio);
    };

    let now = Utc::now();
    separated.push(" update_time = ");
    separated.push_bind_unseparated(now);

    separated.push_unseparated("WHERE id = ");
    query_builder.push_bind(id);

    query_builder
}
