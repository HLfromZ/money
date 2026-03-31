use crate::domain::service::UserService;
use crate::infrastructure::user::sqlite::SqliteUserRepository;
use crate::trigger::http::user::user_public_routes;
use axum::Router;
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

pub fn init_route(url_prefix: &str, pool: Pool<Sqlite>) -> anyhow::Result<Router> {
    let user_router = init_user_route(url_prefix, pool);
    Ok(Router::new().merge(user_router))
}

pub fn init_user_route(url_prefix: &str, pool: Pool<Sqlite>) -> Router {
    let user_repository = Arc::new(SqliteUserRepository::new(pool.clone()));
    let user_service = Arc::new(UserService::new(user_repository));
    Router::new().merge(user_public_routes(url_prefix, user_service))
}
