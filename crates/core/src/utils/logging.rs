//! Shared per-target log levels for the application.
#[cfg(feature = "server")]
use crate::prelude::*;
use tracing::Level;

/// Per-target log level overrides applied to both server and wasm subscribers.
///
/// - Pins noisy dependencies so default `Info` traffic stays readable
pub const LOG_TARGETS: &[(&str, Level)] = &[
    ("cookie", Level::INFO),
    ("dioxus", Level::INFO),
    ("html5ever", Level::INFO),
    ("hyper_util", Level::INFO),
    ("lofty", Level::INFO),
    ("reqwest", Level::INFO),
    ("selectors", Level::INFO),
    ("sqlx", Level::WARN),
    ("warnings", Level::DEBUG),
];

/// Convert a [`tracing::Level`] to a [`LogLevel`].
#[cfg(feature = "server")]
pub(crate) fn log_level_from(level: Level) -> LogLevel {
    match level {
        Level::ERROR => LogLevel::Error,
        Level::WARN => LogLevel::Warn,
        Level::INFO => LogLevel::Info,
        Level::DEBUG => LogLevel::Debug,
        Level::TRACE => LogLevel::Trace,
    }
}
