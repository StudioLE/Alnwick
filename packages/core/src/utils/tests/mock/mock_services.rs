use crate::prelude::*;

/// Factory for creating mock [`ServiceProvider`] instances.
pub struct MockServices {
    /// Mock feeds to insert into the database.
    metadata: Option<MockFeedsFactory>,
    /// Mock HTTP fetch with pre-configured URL mappings.
    mock_http: MockHttpClient,
}

impl MockServices {
    #[must_use]
    pub fn rss_url() -> UrlWrapper {
        UrlWrapper::from_str("https://example.com/mock-feed.xml").expect("URL should parse")
    }

    /// Create a new [`MockServices`] instance with no mock feeds.
    ///
    /// Pre-configures the mock HTTP with sample fixture mappings for the
    /// episode file and image URLs used by [`MockFeedsFactory`].
    #[must_use]
    pub fn new() -> Self {
        let mock_http = MockHttpClient::new()
            .with_file(
                MockFeeds::episode_file_url().as_str(),
                SampleFixtures::mp3(),
            )
            .with_file(MockFeeds::EPISODE_IMAGE_URL, SampleFixtures::png());
        Self {
            metadata: None,
            mock_http,
        }
    }

    /// Set a custom metadata factory for mock feeds.
    #[must_use]
    pub fn with_metadata_factory(mut self, factory: MockFeedsFactory) -> Self {
        self.metadata = Some(factory);
        self
    }

    /// Insert default mock feeds into the database.
    #[must_use]
    pub fn with_metadata(mut self) -> Self {
        self.metadata = Some(MockFeedsFactory::default());
        self
    }

    /// Prime the mock HTTP with RSS feed data.
    ///
    /// - Generates RSS XML from the first mock podcast
    /// - Maps the RSS URL to return `application/xml` for HEAD requests
    /// - Maps episode file and image URLs to sample fixtures
    #[must_use]
    pub fn with_rss_feed(mut self) -> Self {
        let factory = self.metadata.clone().unwrap_or_default();
        self.mock_http = build_rss_mock(self.mock_http, &factory);
        self
    }

    /// Create a new [`ServiceProvider`] instance with mock services.
    ///
    /// - Creates a temporary data directory and stores it in app options
    /// - Creates a temporary sqlite database
    /// - Registers `MockHttpClient` as `dyn HttpFetch` instead of `HttpClient`
    /// - Inserts mock feeds into the database
    pub async fn create(self) -> ServiceProvider {
        trace!("Creating mock services");
        let options = mock_app_options();
        create_data_dir(&options).await;
        let http: Arc<dyn HttpFetch> = Arc::new(self.mock_http);
        let services = ServiceBuilder::new()
            .with_instance(options)
            .with_core()
            .with_instance(http)
            .with_commands()
            .build();
        if let Some(factory) = self.metadata {
            insert_db_feeds(&services, factory).await;
        } else {
            debug!("No mock feeds. Database will be empty");
        }
        services
    }
}

async fn create_data_dir(options: &AppOptions) {
    let data_dir = options.data_dir.as_ref().expect("data dir should be set");
    trace!(path = %data_dir.display(), "Creating temp data dir");
    create_dir_all(&data_dir)
        .await
        .expect("should be able to create temp data dir");
}

async fn insert_db_feeds(services: &ServiceProvider, factory: MockFeedsFactory) {
    trace!(podcasts = ?factory.podcast_count, "Inserting mock feeds to database");
    let metadata = services
        .get_async::<MetadataRepository>()
        .await
        .expect("should be able to get MetadataRepository");
    let mock = factory.create();
    for feed in mock.feeds {
        metadata
            .create_feed(feed)
            .await
            .expect("should be able to save feed");
    }
}

fn build_rss_mock(mock_http: MockHttpClient, factory: &MockFeedsFactory) -> MockHttpClient {
    let mock = factory.clone().create();
    let feed = mock
        .feeds
        .into_iter()
        .next()
        .expect("mock feeds should have at least one podcast");
    let channel = PodcastToRss::execute(feed.clone());
    let rss_xml = channel.to_string();
    let rss_url = MockServices::rss_url();
    mock_http
        .with_string(rss_url.as_str(), rss_xml)
        .with_content_type(rss_url.as_str(), "application/xml")
}

impl Default for MockServices {
    /// Create a new [`MockServices`] instance with mock feeds.
    fn default() -> Self {
        MockServices::new().with_metadata()
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
            .get_async::<AppOptions>()
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
