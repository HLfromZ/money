use crate::api::user::dto::{UserLoginRequest, UserRegisterRequest};
use crate::domain::share::error::AppError;
use crate::domain::user::error::UserError;
use crate::domain::user::repository::UserRepository;
use crate::error_sys;
use crate::util::jwt;
use argon2::password_hash::Error;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
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

    pub async fn login(&self, req: UserLoginRequest) -> Result<String, AppError> {
        let user = self
            .repo
            .select_by_username(&req.username)
            .await?
            .ok_or(UserError::UsernamePasswordIncorrect)?;

        Argon2::default()
            .verify_password(req.password.as_bytes(), user.pwd_hash.as_str())
            .map_err(|e| match e {
                Error::PasswordInvalid => UserError::UsernamePasswordIncorrect.into(),
                _ => {
                    error_sys!("密码校验失败 {}", e);
                    AppError::SystemError
                }
            })?;

        let access_token = jwt::sign_access_token(user.user_id, &user.username)?;
        Ok(access_token)
    }
}
