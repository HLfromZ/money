use crate::api::user::dto::UserRegisterRequest;
use crate::domain::share::error::AppError;
use crate::domain::user::error::UserError;
use crate::domain::user::repository::UserRepository;
use crate::error_sys;
use argon2::{Argon2, PasswordHasher};
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn register(&self, req: UserRegisterRequest) -> Result<(), AppError> {
        if self.repo.select_by_username(&req.username).await?.is_some() {
            return Err(UserError::UsernameAlreadyExists.into());
        }

        let pwd_hash = Argon2::default()
            .hash_password(req.password.as_bytes())
            .map(|hash| hash.to_string())
            .map_err(|e| {
                error_sys!("密码哈希失败 {}", e);
                AppError::SystemError
            })?;

        let user_id = self.repo.insert(req.username.as_str(), pwd_hash).await?;
        info!("👏🏻 新用户 user_id: {}, username: {}", user_id, req.username);

        Ok(())
    }
}
