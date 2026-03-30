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
        self.with_type::<AppOptions>()
            .with_type::<MountProvider>()
            .with_type::<PathProvider>()
            .with_type::<HttpRateLimiter>()
            .with_type::<IpInfoProvider>()
            .with_trait_async::<dyn HttpFetch, HttpClient>()
            .with_type_async::<MetadataRepository>()
            .with_type_async::<AddHandler>()
            .with_type_async::<AddCliCommand>()
            .with_type_async::<FetchCliCommand>()
            .with_type_async::<DownloadCliCommand>()
            .with_type_async::<EmulateCommand>()
            .with_type_async::<CoverCommand>()
    }
}
