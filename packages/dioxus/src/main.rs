use alnwick_core::prelude::{DEFAULT_LOG_LEVEL, get_targets};
use alnwick_dioxus::App;
use dioxus::launch;
use dioxus::logger::tracing::dispatcher::SetGlobalDefaultError;
use dioxus::logger::tracing::level_filters::LevelFilter;
use dioxus::logger::tracing::subscriber::set_global_default;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{Layer, Registry};
use tracing_wasm::WASMLayer;

#[allow(clippy::absolute_paths)]
fn main() {
    init_logger().expect("should be able to init logger");
    launch(App);
}

fn init_logger() -> Result<(), SetGlobalDefaultError> {
    #[cfg(target_arch = "wasm32")]
    {
        wasm_logger()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        server_logger()
    }
}

#[allow(dead_code)]
fn server_logger() -> Result<(), SetGlobalDefaultError> {
    let targets = get_targets();
    let mut layer = layer().compact().without_time().with_target(false);
    if dioxus_cli_config::is_cli_enabled() {
        layer = layer.without_time();
    }
    let layer = layer.with_filter(targets);
    let registry = Registry::default().with(layer);
    set_global_default(registry)
}

#[allow(dead_code)]
fn wasm_logger() -> Result<(), SetGlobalDefaultError> {
    let targets = Targets::new()
        .with_target("dioxus", LevelFilter::INFO)
        .with_default(DEFAULT_LOG_LEVEL);
    let layer = WASMLayer::default().with_filter(targets);
    let reg = Registry::default().with(layer);
    set_global_default(reg)
}
