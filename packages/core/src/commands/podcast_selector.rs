use crate::prelude::*;

/// Resolve [`PodcastOptions`] to a list of podcast slugs.
#[derive(Clone, FromServicesAsync)]
pub struct PodcastSelector {
    metadata: Arc<MetadataRepository>,
}

impl PodcastSelector {
    /// Resolve podcast options to a validated list of slugs.
    ///
    /// - Errors if neither `--podcast` nor `--all-podcasts` was provided.
    /// - Errors if no podcasts exist in the database.
    /// - If `--podcast` was provided, validates the slug exists.
    /// - If `--all-podcasts` was provided, returns all slugs.
    pub async fn execute(
        &self,
        options: &PodcastOptions,
    ) -> Result<Vec<Slug>, Report<PodcastSelectorError>> {
        if options.podcast.is_none() && !options.all_podcasts {
            return Err(Report::new(PodcastSelectorError::NoSelection));
        }
        let all_slugs = self
            .metadata
            .get_all_podcast_slugs()
            .await
            .change_context(PodcastSelectorError::Repository)?;
        if all_slugs.is_empty() {
            return Err(Report::new(PodcastSelectorError::NoPodcasts));
        }
        let Some(slug) = &options.podcast else {
            return Ok(all_slugs);
        };
        if all_slugs.contains(slug) {
            Ok(vec![slug.clone()])
        } else {
            Err(Report::new(PodcastSelectorError::NotFound).attach("podcast", slug.clone()))
        }
    }
}

/// Errors from [`PodcastSelector`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum PodcastSelectorError {
    /// Neither `--podcast` nor `--all-podcasts` was provided.
    #[error("Either --podcast or --all-podcasts must be provided")]
    NoSelection,
    /// Unable to query the database for podcast slugs.
    #[error("Unable to query database for podcast slugs")]
    Repository,
    /// No podcasts found.
    #[error("No podcasts found")]
    NoPodcasts,
    /// Podcast does not exist.
    #[error("Podcast does not exist")]
    NotFound,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[serial]
    async fn execute_single_podcast() {
        // Arrange
        let services = MockServices::default().create().await;
        let selector = services
            .get_async::<PodcastSelector>()
            .await
            .expect("should be able to get selector");
        let options = PodcastOptions {
            podcast: Some(MockFeeds::podcast_slug()),
            all_podcasts: false,
        };

        // Act
        let result = selector.execute(&options).await;

        // Assert
        let slugs = result.assert_ok_debug();
        assert_eq!(slugs, vec![MockFeeds::podcast_slug()]);
    }

    #[tokio::test]
    #[serial]
    async fn execute_all_podcasts() {
        // Arrange
        let services = MockServices::default().create().await;
        let selector = services
            .get_async::<PodcastSelector>()
            .await
            .expect("should be able to get selector");
        let options = PodcastOptions {
            podcast: None,
            all_podcasts: true,
        };

        // Act
        let result = selector.execute(&options).await;

        // Assert
        let slugs = result.assert_ok_debug();
        assert_eq!(
            slugs.len(),
            usize::try_from(MockFeeds::PODCAST_COUNT).expect("should fit")
        );
    }

    #[tokio::test]
    #[serial]
    async fn execute_single_podcast_not_found() {
        // Arrange
        let services = MockServices::default().create().await;
        let selector = services
            .get_async::<PodcastSelector>()
            .await
            .expect("should be able to get selector");
        let options = PodcastOptions {
            podcast: Some(Slug::from_str("nonexistent").expect("should be valid slug")),
            all_podcasts: false,
        };

        // Act
        let result = selector.execute(&options).await;

        // Assert
        let error = result.expect_err("should return NotFound error");
        assert_eq!(error.current_context(), &PodcastSelectorError::NotFound);
    }

    #[tokio::test]
    #[serial]
    async fn execute_no_selection() {
        // Arrange
        let services = MockServices::default().create().await;
        let selector = services
            .get_async::<PodcastSelector>()
            .await
            .expect("should be able to get selector");
        let options = PodcastOptions {
            podcast: None,
            all_podcasts: false,
        };

        // Act
        let result = selector.execute(&options).await;

        // Assert
        let error = result.expect_err("should return NoSelection error");
        assert_eq!(error.current_context(), &PodcastSelectorError::NoSelection);
    }

    #[tokio::test]
    #[serial]
    async fn execute_all_podcasts_empty() {
        // Arrange
        let services = MockServices::new()
            .with_metadata_factory(MockFeedsFactory {
                podcast_count: 0,
                ..MockFeedsFactory::default()
            })
            .create()
            .await;
        let selector = services
            .get_async::<PodcastSelector>()
            .await
            .expect("should be able to get selector");
        let options = PodcastOptions {
            podcast: None,
            all_podcasts: true,
        };

        // Act
        let result = selector.execute(&options).await;

        // Assert
        let error = result.expect_err("should return NoPodcasts error");
        assert_eq!(error.current_context(), &PodcastSelectorError::NoPodcasts);
    }
}
