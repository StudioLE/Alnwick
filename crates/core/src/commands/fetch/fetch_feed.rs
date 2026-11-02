use super::podcast_from_rss::PodcastFromRss;
use crate::prelude::*;
use rss::Channel as RssChannel;

/// Maximum number of feed redirects to follow.
const MAX_FEED_REDIRECTS: usize = 10;

impl FetchHandler {
    /// Fetch and parse a podcast feed without saving.
    ///
    /// - Follows `new_feed_url` redirects (up to 10)
    /// - Sets `feed_url` to the canonical URL
    /// - Resolves Simplecast URLs to RSS feeds
    pub async fn fetch_feed(
        &self,
        slug: &Slug,
        url: &UrlWrapper,
    ) -> Result<PodcastFeed, Report<FetchError>> {
        let (mut feed, url) = self.fetch_feed_with_redirect(slug, url).await?;
        feed.podcast.feed_url = Some(url);
        Ok(feed)
    }

    async fn fetch_feed_with_redirect(
        &self,
        slug: &Slug,
        url: &UrlWrapper,
    ) -> Result<(PodcastFeed, UrlWrapper), Report<FetchError>> {
        let mut visited = HashSet::new();
        let mut current = url.clone();
        let mut i = 0;
        loop {
            trace!(%slug, url = %current, "Fetching feed");
            let feed = self
                .fetch_feed_without_redirect(slug, &current)
                .await
                .change_context(FetchError::Rss)?;
            visited.insert(current.clone());
            let Some(next) = &feed.podcast.new_feed_url else {
                return Ok((feed, current));
            };
            if next == &current {
                return Ok((feed, current));
            }
            if visited.contains(next) {
                return Err(Report::new(FetchError::RedirectLoop).attach_url(next));
            }
            if i >= MAX_FEED_REDIRECTS {
                return Err(
                    Report::new(FetchError::TooManyRedirects).attach("Limit", MAX_FEED_REDIRECTS)
                );
            }
            trace!(%slug, %current, new_feed_url = %next, redirects = i, "Feed includes a `new_feed_url`");
            current = next.clone();
            i += 1;
        }
    }

    async fn fetch_feed_without_redirect(
        &self,
        slug: &Slug,
        url: &UrlWrapper,
    ) -> Result<PodcastFeed, Report<FetchRssError>> {
        let content_type = self
            .http
            .head(url)
            .await
            .change_context(FetchRssError::Xml)?;
        let resolved_url = if is_rss_content_type(&content_type) {
            url.clone()
        } else {
            self.get_simplecast_rss(slug, url)
                .await
                .change_context(FetchRssError::Convert)?
        };
        self.parse_rss(&resolved_url, slug).await
    }

    async fn parse_rss(
        &self,
        url: &UrlWrapper,
        slug: &Slug,
    ) -> Result<PodcastFeed, Report<FetchRssError>> {
        let xml = self
            .http
            .get_string(url)
            .await
            .change_context(FetchRssError::Xml)?;
        let reader = BufReader::new(xml.as_bytes());
        let channel = RssChannel::read_from(reader).change_context(FetchRssError::Parse)?;
        PodcastFromRss::execute(channel, slug.clone()).change_context(FetchRssError::Convert)
    }
}

/// Whether a response with this content type can be parsed as an RSS feed.
///
/// Expects a lowercase media type with parameters such as `charset` removed.
fn is_rss_content_type(content_type: &str) -> bool {
    matches!(
        content_type,
        "application/rss+xml" | "application/xml" | "text/xml"
    )
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[tokio::test]
    #[serial]
    #[ignore = "uses irlpodcast.org"]
    pub async fn fetch_feed_simplecast() {
        // Arrange
        let handler = MockServices::new()
            .create()
            .await
            .get_async::<FetchHandler>()
            .await
            .expect("should be able to get handler");
        let url = UrlWrapper::from_str("https://irlpodcast.org").expect("URL should parse");

        // Act
        let result = handler.fetch_feed(&MockFeeds::podcast_slug(), &url).await;

        // Assert
        let podcast = result.assert_ok_debug();
        assert!(podcast.episodes.len() > 30);
    }

    #[tokio::test]
    #[serial]
    pub async fn fetch_feed_rss() {
        // Arrange
        let handler = MockServices::new()
            .with_rss_feed()
            .create()
            .await
            .get_async::<FetchHandler>()
            .await
            .expect("should be able to get handler");

        // Act
        let result = handler
            .fetch_feed(&MockFeeds::podcast_slug(), &MockServices::rss_url())
            .await;

        // Assert
        let podcast = result.assert_ok_debug();
        assert_yaml_snapshot!(podcast);
    }

    /// Megaphone feeds redirected to Amperwave are served as `application/rss+xml`.
    #[test]
    fn is_rss_content_type_rss_xml() {
        assert!(is_rss_content_type("application/rss+xml"));
    }

    /// Web pages go to the Simplecast scraper instead.
    #[test]
    fn is_rss_content_type_html() {
        assert!(!is_rss_content_type("text/html"));
    }
}
