use http_problem::Result;
use std::sync::Arc;
use tokio_postgres::Client;

use crate::{
    domain::user::{User, UserRepository},
    routes::user::NewUserPayload,
};

pub struct SqlUserRepository {
    pub client: Arc<Client>,
}

#[async_trait::async_trait]
impl UserRepository for SqlUserRepository {
    async fn create_user(&self, user: NewUserPayload) -> Result<()> {
        let row = self
            .client
            .query_one(
                "INSERT INTO users (nickname, email, password) VALUES ($1, $2, $3) RETURNING id",
                &[&user.nickname, &user.email, &user.password],
            )
            .await
            .expect("aaa");
        Ok(())
    }

    async fn update_user(&self, user: User) -> Result<()> {
        todo!()
    }

    async fn get_user_by_id(&self, id: i64) -> Result<User> {
        todo!()
    }

    async fn get_user_by_nickname(&self, nickname: String) -> Result<User> {
        todo!()
    }

    async fn get_user_by_email(&self, email: String) -> Result<User> {
        todo!()
    }

    async fn delete_user(&self, id: i64) -> Result<()> {
        todo!()
    }
}
