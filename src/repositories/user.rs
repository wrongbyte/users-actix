use chrono::{DateTime, Utc};
use http_problem::Result;
use std::sync::Arc;
use tokio_postgres::{Client, Row};

use crate::{
    domain::user::{PublicUser, User, UserRepository},
    routes::user::NewUserPayload,
};

pub struct SqlUserRepository {
    pub client: Arc<Client>,
}

#[async_trait::async_trait]
impl UserRepository for SqlUserRepository {
    async fn create_user(&self, user: NewUserPayload) -> Result<PublicUser> {
        let row = self
            .client
            .query_one(
                "INSERT INTO users (nickname, email, password) VALUES ($1, $2, $3) RETURNING name, nickname, email, bio, creation_time",
                &[&user.nickname, &user.email, &user.password],
            )
            .await
            .expect("Error on query");
        let new_user = get_public_user_from_sql(row);
        Ok(new_user)
    }

    async fn update_user(&self, user: User) -> Result<()> {
        todo!()
    }

    async fn get_user_by_nickname(&self, nickname: String) -> Result<PublicUser> {
        let row = self
            .client
            .query_one(
                "SELECT name, nickname, email, bio, creation_time FROM users WHERE nickname = $1",
                &[&nickname],
            )
            .await
            .expect("Error on query");
        let user = get_public_user_from_sql(row);
        Ok(user)
    }

    async fn get_user_by_email(&self, email: String) -> Result<User> {
        todo!()
    }

    async fn delete_user(&self, id: i64) -> Result<()> {
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
