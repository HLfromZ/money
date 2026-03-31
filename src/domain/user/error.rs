use crate::domain::share::error::AppError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("User not found")]
    NotFound,

    #[error("Username already exists")]
    UsernameAlreadyExists,

    #[error("Username or password incorrect")]
    UsernamePasswordIncorrect,

    #[error("Database error")]
    DBError,
}

impl From<UserError> for AppError {
    fn from(e: UserError) -> AppError {
        match e {
            UserError::NotFound => AppError::BusinessError("用户不存在".to_string()),
            UserError::UsernameAlreadyExists => AppError::BusinessError("用户名已存在".to_string()),
            UserError::UsernamePasswordIncorrect => {
                AppError::BusinessError("用户名或密码错误".to_string())
            }
            UserError::DBError => AppError::SystemError,
        }
    }
}
