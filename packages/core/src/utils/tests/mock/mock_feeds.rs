use crate::prelude::*;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;

/// Factory for generating mock [`PodcastFeed`] for testing.
pub struct MockFeeds {
    pub factory: MockFeedsFactory,
    /// Number of podcasts to generate.
    pub feeds: Vec<PodcastFeed>,
}

impl MockFeeds {
    /// Image URL used for podcast and episode artwork.
    pub const EPISODE_IMAGE_URL: &str =
        "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png";
    /// Number of years of episodes to generate.
    pub const YEAR_COUNT: u32 = 3;
    /// Number of seasons per year.
    pub const SEASONS_PER_YEAR: u32 = 2;
    /// Starting year for episode dates.
    pub const START_YEAR: u32 = 2000;
    /// Number of episodes per season.
    pub const EPISODES_PER_SEASON: u32 = 3;
    /// Number of podcasts to generate.
    pub const PODCAST_COUNT: u32 = 3;
    const EPISODE_FILE_URL: &str = "aHR0cHM6Ly9maWxlcy5mcmVlbXVzaWNhcmNoaXZlLm9yZy9zdG9yYWdlLWZyZWVtdXNpY2FyY2hpdmUtb3JnL3RyYWNrcy9nR1J5M1JmYm1EWE5vOEw1SlBPc0I3ZFBoTXhnbEJKaEw4M2owVHp5Lm1wMw==";

    /// An example podcast key.
    ///
    /// This key will exist if using `MockFeeds::default()` or `MockServices::default()`.
    ///
    /// The slug for this row will be [`MockFeeds::PODCAST_SLUG`] or [`MockFeeds::podcast_slug()`].
    pub const PODCAST_KEY: PodcastKey = 1;
    /// An example episode key
    ///
    /// This key will exist for [`MockFeeds::PODCAST_KEY`] if using `MockFeeds::default()` or `MockServices::default()`.
    pub const EPISODE_KEY: EpisodeKey = 2;
    /// Slug for the first podcast.
    ///
    /// The key will be [`MockFeeds::PODCAST_KEY`].
    const PODCAST_SLUG: &str = "test-0";

    /// Get the slug for the first podcast.
    ///
    /// The key will be [`MockFeeds::PODCAST_KEY`].
    #[must_use]
    pub fn podcast_slug() -> Slug {
        Slug::from_str(Self::PODCAST_SLUG).expect("should be valid slug")
    }

    /// Get the image URL used for episodes.
    #[must_use]
    pub fn image_url() -> UrlWrapper {
        UrlWrapper::from_str(Self::EPISODE_IMAGE_URL).expect("should be valid URL")
    }

    /// Get an example file URL for an episode.
    ///
    /// This is a real URL.
    #[must_use]
    pub(super) fn episode_file_url() -> UrlWrapper {
        let bytes = BASE64_STANDARD
            .decode(Self::EPISODE_FILE_URL)
            .expect("should be valid base64");
        let url = String::from_utf8(bytes).expect("should be valid UTF-8");
        UrlWrapper::from_str(&url).expect("should be valid URL")
    }
}

impl Default for MockFeeds {
    fn default() -> Self {
        MockFeedsFactory::default().create()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn episode_file_url() {
        // Arrange
        // Act
        // Assert
        let _url = MockFeeds::episode_file_url();
    }
}
