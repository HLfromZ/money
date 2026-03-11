use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub user_id: i64,
    pub username: String,
    pub pwd_hash: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}