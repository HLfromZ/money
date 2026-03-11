use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("User not found")]
    NotFound,

    #[error("Username already exists")]
    UsernameAlreadyExists,

    #[error("Username or password incorrect")]
    UsernamePasswordIncorrect,

    #[error("Wrong password")]
    WrongPassword,

    #[error("Database error")]
    DBError,
}
