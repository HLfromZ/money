use std::env;
use std::sync::OnceLock;

pub struct Config {
    pub run_mode: String,
    pub log_level: String,
    pub database_url: String,
    pub server_port: u16,
    pub url_prefix: String,
    pub jwt_secret: Vec<u8>,
    pub jwt_expire_second: i64,
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
        let url_prefix = env::var("URL_PREFIX").unwrap_or_else(|_| "".to_string());

        let jwt_secret = match env::var("JWT_SECRET") {
            Ok(secret) => secret.into_bytes(),
            Err(_) => {
                eprintln!("❌ JWT_SECRET is not set");
                std::process::exit(1);
            }
        };

        let jwt_expire_second = match env::var("JWT_EXPIRE_SECOND") {
            Ok(secret_str) => match secret_str.parse::<i64>() {
                Ok(secret) => secret,
                Err(_) => {
                    eprintln!("❌ JWT_EXPIRE_SECOND is illegal");
                    std::process::exit(1);
                }
            },
            Err(_) => {
                eprintln!("❌ JWT_EXPIRE_SECOND is not set");
                std::process::exit(1);
            }
        };

        let config = Config {
            run_mode,
            log_level,
            database_url,
            server_port: server_port.parse().unwrap(),
            url_prefix,
            jwt_secret,
            jwt_expire_second,
        };

        if CONFIG.set(config).is_err() {
            eprintln!("❌ 配置已初始化")
        };
    }

    pub fn get() -> &'static Config {
        CONFIG.get().expect("❌ 配置未初始化")
    }
}
