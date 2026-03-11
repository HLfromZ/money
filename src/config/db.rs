use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::path::Path;
use std::str::FromStr;
use tracing::info;

pub async fn init_db(database_url: &str) -> anyhow::Result<Pool<Sqlite>> {
    info!("🗄️ 正在连接数据库...");

    let file_path_str = database_url
        .strip_prefix("sqlite://")
        .unwrap_or(database_url);

    let path = Path::new(file_path_str);

    if let Some(parent_dir) = path.parent()
        && !parent_dir.as_os_str().is_empty()
        && !parent_dir.exists()
    {
        info!("🗄️ 数据库目录不存在，开始创建...");
        tokio::fs::create_dir_all(parent_dir).await?;
    }

    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    info!("🗄️ 数据库连接成功，检查数据库结构变更...");

    let before_migration: Vec<i64> = sqlx::query_scalar("SELECT version FROM _sqlx_migrations")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    sqlx::migrate!("./devops/sql").run(&pool).await?;

    let after_migration: Vec<(i64, String)> =
        sqlx::query_as::<_, (i64, String)>("SELECT version, description FROM _sqlx_migrations")
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

    let new_migrations: Vec<String> = after_migration
        .into_iter()
        .filter(|(version, _)| !before_migration.contains(version))
        .map(|(_, description)| description)
        .collect();

    if new_migrations.is_empty() {
        info!("🗄️ 数据库结构无变更");
    } else {
        info!("🗄️ 数据库升级: {:?}", new_migrations);
    }

    info!("🗄️ 数据库初始化完成");

    Ok(pool)
}
