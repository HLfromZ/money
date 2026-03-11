use money::config::db::init_db;
use money::config::log::init_log;
use money::domain::user::model::entity::User;
use money::domain::user::repository::UserRepository;
use money::infrastructure::user::sqlite::SqliteUserRepository;
use sqlx::{Pool, Sqlite};
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

pub static DATABASE_URL: &str = "sqlite://tests/db/money.db";

async fn connect_db() -> Pool<Sqlite> {
    init_db(DATABASE_URL)
        .await
        .expect("failed to create database")
}

#[tokio::test]
async fn user_test() {
    let _log_guard = init_log("debug");
    let pool = connect_db().await;

    let repo = SqliteUserRepository::new(pool);

    let username = "test";
    let pwd_hash = "123";

    let user_id = insert_test(username, pwd_hash, &repo).await;
    info!("User inserted, user_id: {}", user_id);

    let mut user = select_by_id_test(user_id, &repo).await.unwrap();
    let user_by_username = select_by_username_test(username, &repo).await.unwrap();

    assert_eq!(user.user_id, user_by_username.user_id);

    info!("User found: {:?}", user);

    sleep(Duration::from_secs(3)).await;

    user.username = "name_changed".into();
    user.pwd_hash = "456".into();
    update_test(user, &repo).await;

    let user_after_update = select_by_username_test("name_changed", &repo)
        .await
        .unwrap();

    info!("User after update: {:?}", user_after_update);

    delete_test(user_id, &repo).await;
    let user_after_delete = select_by_id_test(user_id, &repo).await;
    info!("User after deletion: {:?}", user_after_delete);
}

async fn insert_test(username: &str, pwd_hash: &str, repo: &SqliteUserRepository) -> i64 {
    repo.insert(username, pwd_hash).await.unwrap()
}

async fn select_by_id_test(user_id: i64, repo: &SqliteUserRepository) -> Option<User> {
    repo.select_by_id(user_id).await.unwrap()
}

async fn select_by_username_test(username: &str, repo: &SqliteUserRepository) -> Option<User> {
    repo.select_by_username(username).await.unwrap()
}

async fn update_test(user: User, repo: &SqliteUserRepository) {
    repo.update(&user).await.unwrap();
}

async fn delete_test(user_id: i64, repo: &SqliteUserRepository) {
    repo.delete(user_id).await.unwrap();
}
