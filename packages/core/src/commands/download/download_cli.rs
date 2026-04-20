use crate::prelude::*;

/// CLI command for batch downloading episodes.
///
/// Queues multiple [`DownloadRequest`] based on filter criteria and
/// executes them concurrently with a progress bar.
#[derive(FromServicesAsync)]
pub struct DownloadCliCommand {
    metadata: Arc<MetadataRepository>,
    selector: Arc<PodcastSelector>,
    cli_runner: Arc<CliRunner>,
}

impl DownloadCliCommand {
    /// Download episodes matching the filter criteria.
    pub async fn execute(&self, options: DownloadOptions) -> Result<(), Report<DownloadCliError>> {
        let slugs = self
            .selector
            .execute(&options.selection)
            .await
            .change_context(DownloadCliError::Selection)?;
        let mut requests = Vec::new();
        for slug in slugs {
            let feed = self
                .metadata
                .get_feed_by_slug(slug, Some(options.filter.clone()))
                .await
                .change_context(DownloadCliError::Repository)?
                .ok_or(DownloadCliError::NoPodcast)?;
            let podcast = feed.podcast.primary_key;
            for episode in feed.episodes.iter() {
                requests.push(DownloadRequest::new(
                    podcast,
                    episode.primary_key,
                    options.replace,
                ));
            }
        }
        let status = self.cli_runner.run(requests).await;
        for (_request, error) in &status.failed {
            warn!("{}", error.render());
        }
        info!(
            "Downloaded audio files for {} episodes",
            status.succeeded.len()
        );
        if !status.failed.is_empty() {
            warn!("Skipped {} episodes due to failures", status.failed.len());
        }
        Ok(())
    }
}

/// Errors from [`DownloadCliCommand`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum DownloadCliError {
    /// Unable to select podcasts.
    #[error("Unable to select podcasts")]
    Selection,
    /// Unable to get podcast feed.
    #[error("Unable to get podcast feed")]
    Repository,
    /// Podcast does not exist.
    #[error("Podcast does not exist")]
    NoPodcast,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::as_conversions, clippy::cast_possible_wrap)]
    use super::*;

    #[tokio::test]
    #[serial]
    pub async fn download_command() {
        // Arrange
        let services = MockServices::default().create().await;
        let command = services
            .get_async::<DownloadCliCommand>()
            .await
            .expect("should be able to get command");
        let options = DownloadOptions {
            selection: PodcastOptions {
                podcast: Some(MockFeeds::podcast_slug()),
                all_podcasts: false,
            },
            filter: FilterOptions {
                year: Some(MockFeeds::START_YEAR as i32),
                season: Some(1),
                ..FilterOptions::default()
            },
            replace: false,
        };
        let _logger = init_test_logger();

        // Act
        let result = command.execute(options).await;

        // Assert
        result.assert_ok_debug();
    }
}
