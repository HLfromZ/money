use std::env;
use std::sync::OnceLock;

pub struct Config {
    pub run_mode: String,
    pub log_level: String,
    pub database_url: String,
    pub server_port: u16,
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn init() {
        // 载入运行环境文件
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".to_string());
        let env_file = format!(".env.{}", run_mode);

        if dotenvy::from_filename(&env_file).is_err() {
            eprintln!("❌ 载入环境文件 {} 失败", env_file);
        } else {
            println!("⚙️ 已载入环境文件 {}", env_file);
        }

        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "debug".to_string());
        let database_url = match env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(_) => {
                eprintln!("❌ DATABASE_URL is not set");
                std::process::exit(1);
            }
        };
        let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "80".to_string());

        let config = Config {
            run_mode,
            log_level,
            database_url,
            server_port: server_port.parse().unwrap(),
        };

        if CONFIG.set(config).is_err() {
            eprintln!("❌ 配置已初始化")
        };
    }

    pub fn get() -> &'static Config {
        CONFIG.get().expect("❌ 配置未初始化")
    }
}
