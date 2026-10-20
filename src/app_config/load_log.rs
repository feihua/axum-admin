use crate::app_config::config::LogConfig;
use tracing::info;
use tracing_appender::rolling;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Layer};

/// 初始化日志
pub fn init_log(log_config: LogConfig) -> tracing_appender::non_blocking::WorkerGuard {
    let directory = log_config.directory; // 日志目录
    let file_name_prefix = log_config.file_name_prefix; // 日志文件名前缀
    let file_appender = rolling::daily(directory, &file_name_prefix);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let timer = ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".to_string());
    let format = fmt::format().with_target(false).with_timer(timer).with_file(true).with_line_number(true);

    let console_layer = fmt::layer()
        .event_format(format.clone())
        .with_writer(std::io::stdout)
        .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG);
    let file_layer = fmt::layer()
        .event_format(format.clone())
        .with_writer(non_blocking)
        .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG);

    tracing_subscriber::registry().with(console_layer).with(file_layer).init();
    info!("init log success: {:?}", file_name_prefix);
    guard
}
