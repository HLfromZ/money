use crate::{get_test_db_pool, info_test, init_test_config, init_test_log};
use axum::Json;
use axum::extract::State;
use money::api::user::dto::{UserLoginRequest, UserRegisterRequest};
use money::domain::service::UserService;
use money::domain::share::error::AppError;
use money::domain::user::repository::UserRepository;
use money::infrastructure::user::sqlite::SqliteUserRepository;
use money::trigger::http::user::{login, register};
use money::util::jwt;
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

#[tokio::test]
pub async fn login_test() {
    init_test_log();
    init_test_config();
    let pool = get_test_db_pool().await;
    let user_repo = Arc::new(SqliteUserRepository::new(pool));
    let user_service = Arc::new(UserService::new(user_repo.clone()));

    let username = unique_username();
    let register_req = UserRegisterRequest {
        username: username.clone(),
        password: "123".to_string(),
    };
    let register_res = register(State(user_service.clone()), Json(register_req)).await;
    assert!(register_res.is_ok());
    info_test!("{:?}", register_res);

    let login_req = UserLoginRequest {
        username: username.clone(),
        password: "123".to_string(),
    };
    let login_res = login(State(user_service.clone()), Json(login_req)).await;
    assert!(login_res.is_ok());
    info_test!("{:?}", login_res);

    let wrong_pwd_req = UserLoginRequest {
        username: username.clone(),
        password: "456".to_string(),
    };
    let wrong_pwd_res = login(State(user_service), Json(wrong_pwd_req)).await;
    assert!(matches!(
        wrong_pwd_res,
        Err(AppError::BusinessError(ref msg)) if msg == "用户名或密码错误"
    ));
    info_test!("{:?}", wrong_pwd_res);

    let inserted_user = user_repo
        .select_by_username(&username)
        .await
        .unwrap()
        .expect("registered user should exist");
    user_repo.delete(inserted_user.user_id).await.unwrap();
}

#[tokio::test]
pub async fn login_nonexistent_user_test() {
    init_test_log();
    init_test_config();
    let pool = get_test_db_pool().await;
    let user_repo = Arc::new(SqliteUserRepository::new(pool));
    let user_service = Arc::new(UserService::new(user_repo));

    let login_req = UserLoginRequest {
        username: unique_username(),
        password: "123".to_string(),
    };
    let login_res = login(State(user_service), Json(login_req)).await;
    assert!(matches!(
        login_res,
        Err(AppError::BusinessError(ref msg)) if msg == "用户名或密码错误"
    ));
    info_test!("{:?}", login_res);
}

#[tokio::test]
pub async fn access_token_verify_test() {
    init_test_log();
    init_test_config();
    let pool = get_test_db_pool().await;
    let user_repo = Arc::new(SqliteUserRepository::new(pool));
    let user_service = Arc::new(UserService::new(user_repo.clone()));

    let username = unique_username();
    let register_req = UserRegisterRequest {
        username: username.clone(),
        password: "123".to_string(),
    };
    let register_res = register(State(user_service.clone()), Json(register_req)).await;
    assert!(register_res.is_ok());
    info_test!("{:?}", register_res);

    let login_req = UserLoginRequest {
        username: username.clone(),
        password: "123".to_string(),
    };
    let access_token = user_service.login(login_req).await.unwrap();
    let claims = jwt::verify_access_token(&access_token).unwrap();
    assert_eq!(claims.sub, "user");
    assert_eq!(claims.username, username);
    assert!(claims.user_id > 0);
    assert!(claims.exp > claims.iat);
    info_test!(
        "claims sub: {}, user_id: {}, username: {}",
        claims.sub,
        claims.user_id,
        claims.username
    );

    let invalid_token_res = jwt::verify_access_token("invalid-token");
    assert!(matches!(
        invalid_token_res,
        Err(AppError::BusinessError(ref msg)) if msg == "登录已失效"
    ));
    info_test!("{:?}", invalid_token_res);

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
