use crate::prelude::*;

const CONCURRENCY: usize = 8;

pub struct DownloadCliCommand {
    metadata: Arc<MetadataRepository>,
    command: Arc<DownloadCommand>,
}

impl Service for DownloadCliCommand {
    type Error = ServiceError;

    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self::new(
            services.get_service().await?,
            services.get_service().await?,
        ))
    }
}

impl DownloadCliCommand {
    #[must_use]
    pub fn new(metadata: Arc<MetadataRepository>, command: Arc<DownloadCommand>) -> Self {
        Self { metadata, command }
    }

    pub async fn execute(&self, options: DownloadOptions) -> Result<(), Report<DownloadCliError>> {
        let feed = self
            .metadata
            .get_feed_by_slug(options.podcast_slug, Some(options.filter))
            .await
            .change_context(DownloadCliError::Repository)?
            .ok_or(DownloadCliError::NoPodcast)?;
        let podcast = feed.podcast.primary_key;
        let results = stream::iter(feed.episodes.into_iter().map(|episode| {
            let request = DownloadRequest::new(podcast, episode.primary_key);
            async move { self.command.execute(request).await }
        }))
        .buffer_unordered(CONCURRENCY)
        .collect::<Vec<_>>()
        .await;
        let mut episodes = Vec::new();
        let mut errors = Vec::new();
        for result in results {
            match result {
                Ok(episode) => episodes.push(episode),
                Err(e) => errors.push(e),
            }
        }
        info!("Downloaded audio files for {} episodes", episodes.len());
        if !errors.is_empty() {
            warn!("Skipped {} episodes due to failures", errors.len());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Error)]
pub enum DownloadCliError {
    #[error("Unable to get podcast")]
    Repository,
    #[error("Podcast does not exist")]
    NoPodcast,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::as_conversions, clippy::cast_possible_wrap)]
    use super::*;

    #[tokio::test]
    #[traced_test]
    pub async fn download_command() {
        // Arrange
        let services = TestServiceProvider::create().await;
        let command = services
            .get_service::<DownloadCliCommand>()
            .await
            .expect("should be able to get command");
        let options = DownloadOptions {
            podcast_slug: MetadataRepositoryExample::podcast_slug(),
            filter: FilterOptions {
                year: Some(MetadataRepositoryExample::START_YEAR as i32),
                season: Some(1),
                ..FilterOptions::default()
            },
        };

        // Act
        let result = command.execute(options).await;

        // Assert
        result.assert_ok_debug();
    }
}
