use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_log(log_level: &str) -> WorkerGuard {
    let timer = tracing_subscriber::fmt::time::OffsetTime::new(
        time::UtcOffset::from_hms(8, 0, 0).expect("❌ 配置 日志 时区 错误"),
        time::format_description::parse(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]",
        )
        .expect("❌ 配置 日志 格式 错误"),
    );

    let console_layer = tracing_subscriber::fmt::layer()
        .with_timer(timer.clone())
        .with_target(false)
        .with_filter(tracing_subscriber::EnvFilter::new(log_level));

    let file_appender = tracing_appender::rolling::Builder::new()
        .rotation(tracing_appender::rolling::Rotation::DAILY) // 设置按天滚动
        .filename_suffix("log")
        .build("./log") // 日志文件夹路径
        .expect("❌ 配置 日志 滚动 错误");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_timer(timer)
        .with_writer(non_blocking)
        .with_filter(tracing_subscriber::EnvFilter::new(log_level));

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();

    guard
}
