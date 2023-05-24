use crate::domain::user::payload::UpdateUserPayload;
use crate::domain::user::{payload::NewUserPayload, PublicUser, User, UserRepository};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio_postgres::{Client, Row};

use super::error::RepositoryError;

pub struct SqlUserRepository {
    pub client: Arc<Client>,
}

#[async_trait::async_trait]
impl UserRepository for SqlUserRepository {
    async fn create_user(&self, user: NewUserPayload) -> Result<PublicUser, RepositoryError> {
        let email = user.email.clone();
        let conflict_message = "There is already an user with this email".to_string();

        if let Some(_) = self.get_user_by_email(email).await? {
            return Err(RepositoryError::Conflict(conflict_message));
        }

        let row = self
            .client
            .query_one(
                "INSERT INTO users (nickname, email, password) VALUES ($1, $2, $3) RETURNING name, nickname, email, bio, creation_time",
                &[&user.nickname, &user.email, &user.password],
            )
            .await?;
        let new_user = get_public_user_from_sql(row);
        Ok(new_user)
    }

    async fn update_user(
        &self,
        id: i32,
        update_payload: UpdateUserPayload,
    ) -> Result<(), RepositoryError> {

        Ok(())
    }

    async fn get_user_by_nickname(
        &self,
        nickname: String,
    ) -> Result<Option<PublicUser>, RepositoryError> {
        let row = self
            .client
            .query_opt(
                "SELECT name, nickname, email, bio, creation_time FROM users WHERE nickname = $1",
                &[&nickname],
            )
            .await?;

        let mut result = None;
        if let Some(user) = row {
            result = Some(get_public_user_from_sql(user))
        }
        Ok(result)
    }

    async fn get_user_by_id(&self, id: i32) -> Result<Option<PublicUser>, RepositoryError> {
        let row = self
            .client
            .query_opt(
                "SELECT name, nickname, email, bio, creation_time FROM users WHERE id = $1",
                &[&id],
            )
            .await?;
        if let Some(user) = row {
            let public_user = get_public_user_from_sql(user);
            return Ok(Some(public_user));
        }
        Ok(None)
    }

    async fn get_user_by_email(
        &self,
        email: String,
    ) -> Result<Option<PublicUser>, RepositoryError> {
        let row = self
            .client
            .query_opt(
                "SELECT name, nickname, email, bio, creation_time FROM users WHERE email = $1",
                &[&email],
            )
            .await?;

        if let Some(user) = row {
            let public_user = get_public_user_from_sql(user);
            return Ok(Some(public_user));
        }
        Ok(None)
    }

    async fn delete_user(&self, id: i32) -> Result<(), RepositoryError> {
        todo!()
    }
}

fn get_public_user_from_sql(row: Row) -> PublicUser {
    let name: Option<String> = row.get(0);
    let nickname: String = row.get(1);
    let email: String = row.get(2);
    let bio: Option<String> = row.get(3);
    let creation_timestamp: chrono::NaiveDateTime = row.get(4);
    let creation_time: DateTime<Utc> = DateTime::from_utc(creation_timestamp, Utc);
    PublicUser {
        name,
        nickname,
        email,
        bio,
        creation_time,
    }
}
