use std::io::stderr;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt::format;

const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::TRACE;

pub fn init_logger() {
    let filter = EnvFilter::builder()
        .with_default_directive(DEFAULT_LOG_LEVEL.into())
        .from_env_lossy();
    let format = format().with_target(false);
    tracing_subscriber::fmt()
        .with_writer(stderr)
        .with_env_filter(filter)
        .event_format(format)
        .init();
}
