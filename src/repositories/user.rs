use http_problem::Result;

use crate::domain::user::{UserRepository, User};

pub struct SqlUserRepository;

#[async_trait::async_trait]
impl UserRepository for SqlUserRepository {
    async fn create_user(&self, user: User) -> Result<()> {
        todo!()
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
