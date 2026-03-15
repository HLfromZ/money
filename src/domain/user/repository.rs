use crate::domain::user::error::UserError;
use crate::domain::user::model::entity::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn select_by_id(&self, id: i64) -> Result<Option<User>, UserError>;
    async fn select_by_username(&self, username: &str) -> Result<Option<User>, UserError>;
    async fn insert(&self, username: &str, pwd_hash: String) -> Result<i64, UserError>;
    async fn update(&self, user: &User) -> Result<(), UserError>;
    async fn delete(&self, user_id: i64) -> Result<(), UserError>;
}
