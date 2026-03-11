use crate::domain::user::error::UserError;
use crate::domain::user::model::entity::User;
use crate::domain::user::repository::UserRepository;
use async_trait::async_trait;
use sqlx::{Error, Pool, Sqlite};
use tracing::{error, info};

pub struct SqliteUserRepository {
    pool: Pool<Sqlite>,
}

impl SqliteUserRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    fn map_err(e: Error) -> UserError {
        match e {
            Error::RowNotFound => UserError::NotFound,
            Error::Database(e) => {
                if e.code().unwrap_or_default().eq("2067") {
                    UserError::UsernameAlreadyExists
                } else {
                    UserError::DBError
                }
            }
            _ => UserError::DBError,
        }
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn select_by_id(&self, id: i64) -> Result<Option<User>, UserError> {
        sqlx::query_as::<_, User>("SELECT * FROM user WHERE user_id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(Self::map_err)
    }

    async fn select_by_username(&self, username: &str) -> Result<Option<User>, UserError> {
        sqlx::query_as::<_, User>("SELECT * FROM user WHERE username = $1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await
            .map_err(Self::map_err)
    }

    async fn insert(&self, username: &str, pwd_hash: &str) -> Result<i64, UserError> {
        let user_id: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO user (username, pwd_hash)
            VALUES ($1, $2)
            RETURNING user_id
                  "#,
        )
        .bind(username)
        .bind(pwd_hash)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("user 插入 失败: {}", e);
            Self::map_err(e)
        })?;
        info!(
            "User inserted, user_id: {}, username: {}",
            user_id, username
        );
        Ok(user_id)
    }

    async fn update(&self, user: &User) -> Result<(), UserError> {
        let _ = sqlx::query(
            r#"
            UPDATE user
            SET username = $1, pwd_hash = $2, update_time = DATETIME(CURRENT_TIMESTAMP, 'localtime')
            WHERE user_id = $3"#,
        )
        .bind(&user.username)
        .bind(&user.pwd_hash)
        .bind(&user.user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!("user 更新 失败");
            Self::map_err(e)
        })?;
        info!(
            "User updated, user_id: {}, username: {}",
            user.user_id, user.username
        );
        Ok(())
    }

    async fn delete(&self, user_id: i64) -> Result<(), UserError> {
        let _ = sqlx::query("DELETE FROM user WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("user 删除 失败");
                Self::map_err(e)
            })?;
        info!("User deleted, user_id: {}", user_id);
        Ok(())
    }
}
