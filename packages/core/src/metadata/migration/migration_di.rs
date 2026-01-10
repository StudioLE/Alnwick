use crate::prelude::*;
use std::sync::OnceLock;

/// Static injection of [`PathProvider`] for migrations.
///
/// This is necessary as [`Migrator::migrations()`] is static and cannot receive
/// dependencies via normal DI. This value is set by [`MetadataRepository::migrate()`].
pub(super) static PATH_PROVIDER: OnceLock<Arc<PathProvider>> = OnceLock::new();

/// Set the [`PathProvider`] for migrations to use.
///
/// - In production, returns an error if already set
/// - In tests, logs a warning but returns `Ok` to allow parallel test execution
#[cfg(not(test))]
pub(crate) fn set_path_provider(
    paths: Arc<PathProvider>,
) -> Result<(), MetadataRepositoryCreateError> {
    PATH_PROVIDER
        .set(paths)
        .map_err(|_| MetadataRepositoryCreateError::MigrationPathProvider)
}

/// Set the [`PathProvider`] for migrations to use.
///
/// - In production, returns an error if already set
/// - In tests, logs a warning but returns `Ok` to allow parallel test execution
#[cfg(test)]
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn set_path_provider(
    paths: Arc<PathProvider>,
) -> Result<(), MetadataRepositoryCreateError> {
    if PATH_PROVIDER.set(paths).is_err() {
        warn!(
            "Failed to set PATH_PROVIDER in migration. This is likely caused by multiple tests running in parallel. In most circumstances it's not an issue as the migration does not affect any test files."
        );
    }
    Ok(())
}
