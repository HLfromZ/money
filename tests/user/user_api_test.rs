use crate::{get_test_db_pool, info_test, init_test_log};
use axum::Json;
use axum::extract::State;
use money::api::user::dto::UserRegisterRequest;
use money::domain::service::UserService;
use money::domain::user::repository::UserRepository;
use money::infrastructure::user::sqlite::SqliteUserRepository;
use money::trigger::http::user::register;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub static DATABASE_URL: &str = "sqlite://tests/db/money.db";

#[tokio::test]
pub async fn register_test() {
    init_test_log();
    let pool = get_test_db_pool().await;
    let user_repo = Arc::new(SqliteUserRepository::new(pool));
    let user_service = Arc::new(UserService::new(user_repo.clone()));

    let user = UserRegisterRequest {
        username: "".to_string(),
        password: "".to_string(),
    };

    let r1 = register(State(user_service.clone()), Json(user)).await;
    assert!(r1.is_err());
    info_test!("{:?}", r1);

    let username = unique_username();
    let user = UserRegisterRequest {
        username: username.clone(),
        password: "123".to_string(),
    };
    let r2 = register(State(user_service), Json(user)).await;
    assert!(r2.is_ok());
    info_test!("{:?}", r2);

    let inserted_user = user_repo
        .select_by_username(&username)
        .await
        .unwrap()
        .expect("registered user should exist");
    user_repo.delete(inserted_user.user_id).await.unwrap();
}

fn unique_username() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    format!("reg{:016x}", nanos & 0xffffffffffffffff)
}
