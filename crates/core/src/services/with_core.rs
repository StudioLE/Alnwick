//! Registration of all core services on a [`ServiceBuilder`].
use crate::prelude::*;

/// Default log level when `--log-level` is not specified.
const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Info;

/// Register all core services.
pub trait WithCore: Sized {
    /// Register all core services.
    #[must_use]
    fn with_core(self) -> Self;
}

impl WithCore for ServiceBuilder {
    fn with_core(self) -> Self {
        self.with_logging(logger_factory)
            .with_type::<AppOptions>()
            .with_type::<MountProvider>()
            .with_type::<PathProvider>()
            .with_type::<HttpRateLimiter>()
            .with_type::<IpInfoProvider>()
            .with_trait_async::<dyn HttpFetch, HttpClient>()
            .with_type_async::<MetadataRepository>()
            .with_type_async::<PodcastSelector>()
            .with_type_async::<CliRunner>()
            .with_type_async::<AddHandler>()
            .with_type_async::<AddCliCommand>()
            .with_type_async::<FetchCliCommand>()
            .with_type_async::<DownloadCliCommand>()
            .with_type_async::<EmulateCliCommand>()
            .with_type_async::<CoverCliCommand>()
            .with_type_async::<SubcommandHandler>()
    }
}

/// Default [`Logger`] factory used by [`WithCore::with_core`].
///
/// - Reads `--log-level` from [`CliArgs`] when registered, falls back to [`LogLevel::Info`]
/// - Per-target overrides are drawn from [`LOG_TARGETS`]
#[expect(
    clippy::unnecessary_wraps,
    reason = "signature dictated by WithLogging::with_logging fn pointer bound"
)]
fn logger_factory(services: &ServiceProvider) -> Result<Logger, Report<ResolveError>> {
    let level = services
        .get::<CliArgs>()
        .ok()
        .and_then(|args| args.log_level)
        .unwrap_or(DEFAULT_LOG_LEVEL);
    let mut builder = LoggerBuilder::new().with_level(level);
    for (target, level) in LOG_TARGETS {
        builder = builder.with_target(*target, log_level_from(*level));
    }
    Ok(builder.build())
}
