use money::config::env::Config;
use money::config::log;
use tracing::info;

#[tokio::main]
async fn main() {
    Config::init();
    let config = Config::get();
    let _log_guard = log::init_log(&config.log_level);
    info!("⚙️ 服务启动中, 环境 {}", config.run_mode);
}
