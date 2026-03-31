use money::config::env::Config;
use money::config::db::init_db;
use money::config::log::init_log;
use sqlx::{Pool, Sqlite};
use std::sync::Once;
use tokio::sync::OnceCell;

pub mod user;

static INIT: Once = Once::new();
static CONFIG_INIT: Once = Once::new();
static JWT_ENV_INIT: Once = Once::new();
static DB_POOL: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

pub fn init_test_log() {
    INIT.call_once(|| {
        let guard = init_log("info");
        std::mem::forget(guard);
    })
}

fn init_test_jwt_env() {
    JWT_ENV_INIT.call_once(|| unsafe {
        std::env::set_var("JWT_SECRET", "test-jwt-secret");
        std::env::set_var("JWT_EXPIRE_SECOND", "3600");
    });
}

pub fn init_test_config() {
    init_test_jwt_env();
    CONFIG_INIT.call_once(Config::init);
}

pub async fn get_test_db_pool() -> Pool<Sqlite> {
    DB_POOL
        .get_or_init(|| async {
            init_db("sqlite://tests/db/money.db")
                .await
                .expect("failed to create database")
        })
        .await
        .clone()
}

#[macro_export]
macro_rules! info_test {
    ($($arg:tt)*) => { tracing::info!("🧐 {}", format_args!($($arg)*)) };
}

#[macro_export]
macro_rules! error_test {
    ($($arg:tt)*) => { tracing::error!("🧐 {}", format_args!($($arg)*)) };
}
