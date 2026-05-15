//! WASM logging setup for the Dioxus web client.
use alnwick_core::prelude::*;
use tracing::subscriber::set_global_default;
use tracing_subscriber::filter::{LevelFilter, Targets};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{Layer, Registry};
use tracing_wasm::WASMLayer;

/// Install the global tracing subscriber for the WASM client.
pub fn init_wasm() {
    let mut filter = Targets::new().with_default(LevelFilter::INFO);
    for (target, level) in LOG_TARGETS {
        filter = filter.with_target(*target, LevelFilter::from(*level));
    }
    let layer = WASMLayer::default().with_filter(filter);
    let registry = Registry::default().with(layer);
    set_global_default(registry).expect("set global default");
}
