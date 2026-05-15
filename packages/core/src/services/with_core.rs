//! Registration of all core services on a [`ServiceBuilder`].
use crate::prelude::*;

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
    }
}

/// Default [`Logger`] factory used by [`WithCore::with_core`].
///
/// - Default level is [`LogLevel::Info`]
/// - Per-target overrides are drawn from [`LOG_TARGETS`]
#[expect(
    clippy::unnecessary_wraps,
    reason = "signature dictated by WithLogging::with_logging fn pointer bound"
)]
fn logger_factory(_services: &ServiceProvider) -> Result<Logger, Report<ResolveError>> {
    let mut builder = LoggerBuilder::new().with_level(LogLevel::Info);
    for (target, level) in LOG_TARGETS {
        builder = builder.with_target(*target, log_level_from(*level));
    }
    Ok(builder.build())
}
