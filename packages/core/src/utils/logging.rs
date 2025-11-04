use std::io::stderr;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::TRACE;

pub fn init_logger() {
    let targets = Targets::new()
        .with_target("cookie", LevelFilter::OFF)
        .with_target("html5ever", LevelFilter::OFF)
        .with_target("hyper_uti", LevelFilter::OFF)
        .with_target("reqwest", LevelFilter::OFF)
        .with_target("selectors", LevelFilter::OFF)
        .with_default(DEFAULT_LOG_LEVEL);
    let layer = layer()
        .compact()
        .with_writer(stderr)
        .with_target(false)
        .with_filter(targets);
    tracing_subscriber::registry().with(layer).init();
}
