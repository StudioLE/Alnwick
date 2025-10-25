use crate::prelude::*;
use super::error::ScrapeError;
use super::error::ScrapeRssError;
use super::options::ScrapeOptions;

pub struct ScrapeCommand {
    pub(super) http: HttpClient,
    pub(super) metadata: MetadataStore,
}

impl ScrapeCommand {
    #[must_use]
    pub fn new(http: HttpClient, metadata: MetadataStore) -> Self {
        Self { http, metadata }
    }

    pub async fn execute(&self, options: ScrapeOptions) -> Result<Podcast, ScrapeError> {
        let content_type = self
            .http
            .head(&options.url)
            .await
            .map_err(ScrapeError::Head)?;
        let podcast = match content_type.as_str() {
            "application/xml" => self.execute_rss(&options).await.map_err(ScrapeError::Rss)?,
            _ => self
                .execute_simplecast(&options)
                .await
                .map_err(ScrapeError::Simplecast)?,
        };
        info!("{} {} episodes", "Fetched".bold(), podcast.episodes.len());
        self.metadata.put(&podcast).map_err(ScrapeError::Save)?;
        Ok(podcast)
    }

    pub(super) async fn execute_rss(
        &self,
        options: &ScrapeOptions,
    ) -> Result<Podcast, ScrapeRssError> {
        let path = self
            .http
            .get(&options.url, Some(RSS_EXTENSION))
            .await
            .map_err(ScrapeRssError::Xml)?;
        let file = File::open(&path)
            .map_err(|e| ScrapeRssError::IO(options.podcast_id.clone(), path.clone(), e))?;
        let reader = BufReader::new(file);
        let channel = RssChannel::read_from(reader).map_err(ScrapeRssError::Parse)?;
        let mut podcast: Podcast = channel.try_into().map_err(ScrapeRssError::Convert)?;
        podcast.id = options.podcast_id.clone();
        Ok(podcast)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn scrape_command_simplecast() {
        // Arrange
        let _ = init_logging();
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");
        let command = ScrapeCommand::new(services.http, services.metadata);
        let options = ScrapeOptions {
            podcast_id: "irl".to_owned(),
            url: Url::parse("https://irlpodcast.org").expect("URL should parse"),
        };

        // Act
        let result = command.execute(options).await;

        // Assert
        let podcast = result.assert_ok();
        assert!(podcast.episodes.len() > 30);
    }

    #[tokio::test]
    pub async fn scrape_command_rss() {
        // Arrange
        let _ = init_logging();
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");
        let command = ScrapeCommand::new(services.http, services.metadata);
        let options = ScrapeOptions {
            podcast_id: "irl-rss".to_owned(),
            url: Url::parse("https://feeds.simplecast.com/lP7owBq8").expect("URL should parse"),
        };

        // Act
        let result = command.execute(options).await;

        // Assert
        let podcast = result.assert_ok();
        assert!(podcast.episodes.len() > 30);
    }
}
