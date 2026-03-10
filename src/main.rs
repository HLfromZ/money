use money::config::env::Config;
use money::config::{db, log};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    Config::init();
    let config = Config::get();

    let _log_guard = log::init_log(&config.log_level);

    info!("⚙️ 服务启动中, 环境 {}", config.run_mode);

    let pool = match db::init_db(&config.database_url).await {
        Ok(pool) => pool,
        Err(e) => {
            error!("❌ 数据库初始化失败 {}", e);
            std::process::exit(-1);
        }
    };

    loop {}
}
