#[macro_export]
macro_rules! info_conf {
    ($($arg:tt)*) => { tracing::info!("⚙️ {}", format_args!($($arg)*)) };
}

#[macro_export]
macro_rules! info_db {
    ($($arg:tt)*) => { tracing::info!("🗄️ {}", format_args!($($arg)*)) };
}

#[macro_export]
macro_rules! error_db {
    ($($arg:tt)*) => { tracing::error!("🗄️ {}", format_args!($($arg)*)) };
}

#[macro_export]
macro_rules! info_api {
    ($($arg:tt)*) => { tracing::info!("🌐 {}", format_args!($($arg)*)) };
}

#[macro_export]
macro_rules! error_sys {
    ($($arg:tt)*) => { tracing::error!("❌ {}", format_args!($($arg)*)) };
}
