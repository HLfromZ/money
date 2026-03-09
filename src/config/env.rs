use std::env;
use std::sync::OnceLock;

pub struct Config {
    pub run_mode: String,
    pub log_level: String,
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

        let config = Config {
            run_mode,
            log_level,
        };

        if CONFIG.set(config).is_err() {
            eprintln!("❌ 配置已初始化")
        };
    }

    pub fn get() -> &'static Config {
        CONFIG.get().expect("❌ 配置未初始化")
    }
}
