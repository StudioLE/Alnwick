use crate::prelude::*;

/// Factory for creating mock [`ServiceProvider`] instances.
pub struct MockServices {
    feeds: Option<MockFeedsFactory>,
}

impl MockServices {
    /// Create a new [`MockServices`] instance with no mock feeds.
    #[must_use]
    pub fn new() -> Self {
        Self { feeds: None }
    }

    /// Create a new [`ServiceProvider`] instance with mock services.
    ///
    /// - Creates a temporary data directory and stores it in app options
    /// - Creates a temporary sqlite database
    /// - Inserts mock feeds into the database.
    pub async fn create(self) -> ServiceProvider {
        trace!("Creating mock services");
        let options = mock_app_options();
        let data_dir = options.data_dir.as_ref().expect("data dir should be set");
        trace!(path = %data_dir.display(), "Creating temp data dir");
        create_dir_all(&data_dir)
            .await
            .expect("should be able to create temp data dir");
        let services = ServiceProvider::new()
            .with_instance(options)
            .with_commands()
            .await
            .expect("should be able to create services with commands");
        let metadata = services
            .get_service::<MetadataRepository>()
            .await
            .expect("should be able to get MetadataRepository");
        if let Some(factory) = self.feeds {
            trace!(podcasts = ?factory.podcast_count, "Inserting mock feeds");
            let mock = factory.create();
            for feed in mock.feeds {
                metadata
                    .create_feed(feed)
                    .await
                    .expect("should be able to save feed");
            }
        } else {
            debug!("No mock feeds. Database will be empty");
        }
        services
    }
}

impl Default for MockServices {
    /// Create a new [`MockServices`] instance with mock feeds.
    fn default() -> Self {
        MockServices {
            feeds: Some(MockFeedsFactory::default()),
        }
    }
}

fn mock_app_options() -> AppOptions {
    let temp_dir = TempDirectory::default()
        .create()
        .expect("should be able to create temp dir");
    let data_dir = temp_dir.join("data");
    AppOptions {
        data_dir: Some(data_dir),
        ..AppOptions::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn service_provider_create() {
        // Arrange
        let services = MockServices::default().create().await;

        // Act
        let options = services
            .get_service::<AppOptions>()
            .await
            .expect("should be able to get options");

        // Assert
        let data_dir = options
            .as_ref()
            .clone()
            .data_dir
            .expect("should have data dir");
        assert!(data_dir.exists());
        assert!(data_dir.components().any(|component| {
            let component = component.as_os_str().to_str().unwrap_or_default();
            component.contains("service_provider_create")
        }));
    }
}
