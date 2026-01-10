use crate::prelude::*;
use chrono::{NaiveDate, NaiveTime, TimeZone};

/// Factory for generating mock [`PodcastFeed`] for testing.
#[derive(Clone, Debug)]
pub struct MockFeedsFactory {
    /// Number of years of episodes to generate.
    pub year_count: u32,
    /// Number of seasons per year.
    pub seasons_per_year: u32,
    /// Starting year for episode dates.
    pub start_year: u32,
    /// Number of episodes per season.
    pub episodes_per_season: u32,
    /// Number of podcasts to generate.
    pub podcast_count: u32,
    pub edit_podcast: Option<fn(&mut PodcastInfo)>,
    pub edit_episode: Option<fn(&mut EpisodeInfo)>,
}

impl MockFeedsFactory {
    /// Create the mock feeds
    #[must_use]
    pub fn create(self) -> MockFeeds {
        let source_url = MockFeeds::episode_file_url();
        let image_url = MockFeeds::image_url();
        let mut feeds = Vec::new();
        for podcast_index in 0..self.podcast_count {
            let mut episodes = Vec::new();
            let slug =
                Slug::from_str(&format!("test-{podcast_index}")).expect("should be valid slug");
            let mut podcast = PodcastInfo {
                title: format!("Podcast {podcast_index}"),
                slug: slug.clone(),
                ..PodcastInfo::example()
            };
            if let Some(edit) = &self.edit_podcast {
                edit(&mut podcast);
            }
            for year_i in 0..self.year_count {
                let year = self.start_year + year_i;
                for season_i in 1..=self.seasons_per_year {
                    let season = year_i * self.seasons_per_year + season_i;
                    for episode in 1..=self.episodes_per_season {
                        let ordinal = season_i * 100 + episode * 7;
                        let mut episode = EpisodeInfo {
                            title: format!("S{season:02}E{episode:02} of {slug}"),
                            published_at: date(year, ordinal),
                            season: Some(season),
                            episode: Some(episode),
                            source_url: source_url.clone(),
                            image: Some(image_url.clone()),
                            ..EpisodeInfo::example()
                        };
                        if let Some(edit) = &self.edit_episode {
                            edit(&mut episode);
                        }
                        episodes.push(episode);
                    }
                }
            }
            feeds.push(PodcastFeed { podcast, episodes });
        }
        MockFeeds {
            factory: self,
            feeds,
        }
    }
}

impl Default for MockFeedsFactory {
    fn default() -> Self {
        Self {
            year_count: MockFeeds::YEAR_COUNT,
            seasons_per_year: MockFeeds::SEASONS_PER_YEAR,
            start_year: MockFeeds::START_YEAR,
            episodes_per_season: MockFeeds::EPISODES_PER_SEASON,
            podcast_count: MockFeeds::PODCAST_COUNT,
            edit_podcast: None,
            edit_episode: None,
        }
    }
}

#[allow(clippy::as_conversions, clippy::cast_possible_wrap)]
fn date(year: u32, ordinal: u32) -> DateTime<FixedOffset> {
    let date = NaiveDate::from_yo_opt(year as i32, ordinal).expect("should be a valid date");
    let datetime = NaiveDateTime::new(
        date,
        NaiveTime::from_hms_opt(0, 0, 0).expect("should be a valid time"),
    );
    let offset = FixedOffset::east_opt(0).expect("should be a valid offset");
    offset.from_utc_datetime(&datetime)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _create() {
        // Arrange
        let factory = MockFeedsFactory::default();

        // Act
        let mock = factory.create();

        // Assert
        assert_yaml_snapshot!(mock.feeds);
    }
}
