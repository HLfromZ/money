use money::config::env::Config;
use money::config::{db, log, route};
use money::{error_sys, info_conf};
use tracing::info;

#[tokio::main]
async fn main() {
    Config::init();
    let config = Config::get();

    let _log_guard = log::init_log(&config.log_level);

    info_conf!("服务启动中, 环境 {}", config.run_mode);

    let pool = match db::init_db(&config.database_url).await {
        Ok(pool) => pool,
        Err(e) => {
            error_sys!("数据库初始化失败 {}", e);
            std::process::exit(-1);
        }
    };

    let router = match route::init_route(&config.url_prefix, pool) {
        Ok(router) => router,
        Err(e) => {
            error_sys!("路由初始化失败 {}", e);
            std::process::exit(-1);
        }
    };

    let addr = format!("0.0.0.0:{}", config.server_port);
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(_) => {
            error_sys!("绑定端口失败");
            std::process::exit(-1);
        }
    };

    info!("🟢 服务启动中, 端口: {}", config.server_port);
    axum::serve(listener, router)
        .await
        .expect("❌ 服务启动失败");
}
