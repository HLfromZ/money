use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UserRegisterRequest {
    #[validate(length(min = 1, max = 20, message = "length: 1-20"))]
    pub username: String,
    #[validate(length(min = 1, max = 64, message = "length: 1-64"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct UserLoginRequest {
    #[validate(length(min = 1, max = 20, message = "length: 1-20"))]
    pub username: String,
    #[validate(length(min = 1, max = 64, message = "length: 1-64"))]
    pub password: String,
}
