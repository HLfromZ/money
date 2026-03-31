use crate::config::env::Config;
use crate::domain::share::auth::Claims;
use crate::domain::share::error::AppError;
use crate::error_sys;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};

pub fn sign_access_token(user_id: i64, username: &str) -> Result<String, AppError> {
    let now = Utc::now();
    let expiration = now + Duration::seconds(*jwt_expire_second());

    let claims = Claims {
        sub: "user".to_string(),
        user_id,
        username: username.to_string(),
        iat: now.timestamp() as usize,
        exp: expiration.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret()),
    )
    .map_err(|e| {
        error_sys!("生成 access token 失败 {}", e);
        AppError::SystemError
    })
}

pub fn verify_access_token(token: &str) -> Result<Claims, AppError> {
    let validation = Validation::new(Algorithm::HS256);
    decode::<Claims>(token, &DecodingKey::from_secret(jwt_secret()), &validation)
        .map(|token_data| token_data.claims)
        .map_err(|e| {
            error_sys!("校验 access token 失败 {}", e);
            AppError::BusinessError("登录已失效".to_string())
        })
}

fn jwt_secret() -> &'static [u8] {
    &Config::get().jwt_secret
}

fn jwt_expire_second() -> &'static i64 {
    &Config::get().jwt_expire_second
}
