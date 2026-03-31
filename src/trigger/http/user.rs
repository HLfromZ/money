use crate::api::user::dto::{UserLoginRequest, UserRegisterRequest};
use crate::domain::service::UserService;
use crate::domain::share::error::AppError;
use crate::domain::share::response::Response;
use crate::info_api;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use std::sync::Arc;
use validator::Validate;

pub async fn register(
    State(user_service): State<Arc<UserService>>,
    Json(req): Json<UserRegisterRequest>,
) -> Result<Response<()>, AppError> {
    info_api!("用户注册 username: {}", req.username);
    req.validate()?;
    user_service.register(req).await?;
    Ok(Response::create_msg("注册成功"))
}

pub async fn login(
    State(user_service): State<Arc<UserService>>,
    Json(req): Json<UserLoginRequest>,
) -> Result<Response<String>, AppError> {
    info_api!("用户登录 username: {}", req.username);
    req.validate()?;
    let access_token = user_service.login(req).await?;
    Ok(Response::ok_data_msg(access_token, "登录成功"))
}

pub fn user_public_routes(url_prefix: &str, user_service: Arc<UserService>) -> Router {
    Router::new()
        .route(&format!("{}/user/register", url_prefix), post(register))
        .route(&format!("{}/user/login", url_prefix), post(login))
        .with_state(user_service)
}
