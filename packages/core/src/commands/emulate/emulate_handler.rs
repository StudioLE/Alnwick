use super::to_rss::PodcastToRss;
use crate::prelude::*;
use rss::Item as RssItem;

/// Generate emulated RSS feeds for a podcast's downloaded episodes.
#[derive(Clone, FromServicesAsync)]
pub struct EmulateHandler {
    options: Arc<AppOptions>,
    paths: Arc<PathProvider>,
    metadata: Arc<MetadataRepository>,
}

#[async_trait]
impl Execute<EmulateRequest, EmulateResponse, Report<EmulateError>> for EmulateHandler {
    /// Execute the emulate handler.
    async fn execute(
        &self,
        request: &EmulateRequest,
    ) -> Result<EmulateResponse, Report<EmulateError>> {
        let feed = self
            .metadata
            .get_feed_by_slug(request.slug.clone(), None)
            .await
            .change_context(EmulateError::Repository)?
            .ok_or(EmulateError::NoPodcast)?;
        let feeds: Vec<_> = self
            .save_feeds(&feed)
            .await?
            .into_iter()
            .flatten()
            .collect();
        let feed_count = feeds.len();
        info!(feed_count, "Created rss feeds");
        Ok(EmulateResponse { feed_count })
    }
}

impl EmulateHandler {
    async fn save_feeds(
        &self,
        feed: &PodcastFeed,
    ) -> Result<Vec<Option<PathBuf>>, Report<EmulateError>> {
        let mut paths = Vec::new();
        paths.push(self.save_feed(feed, None, None).await?);
        let mut feed = feed.clone();
        let groups = group_by_season(take(&mut feed.episodes));
        for (season, episodes) in groups {
            let mut p = feed.clone();
            p.episodes = episodes;
            paths.push(self.save_feed(&p, season, None).await?);
            let year_groups = group_by_year(take(&mut p.episodes));
            for (year, episodes) in year_groups {
                p.episodes = episodes;
                paths.push(self.save_feed(&p, season, Some(year)).await?);
            }
        }
        Ok(paths)
    }

    async fn save_feed(
        &self,
        feed: &PodcastFeed,
        season: Option<u32>,
        year: Option<i32>,
    ) -> Result<Option<PathBuf>, Report<EmulateError>> {
        let mut channel = PodcastToRss::execute(feed.clone());
        if let Some(itunes_ext) = channel.itunes_ext.as_mut() {
            itunes_ext.new_feed_url = None;
        }
        let items = take(&mut channel.items);
        for item in items {
            let episode = item
                .title
                .clone()
                .unwrap_or_else(|| item.guid.clone().map(|x| x.value).unwrap_or_default());
            match self.replace_enclosure(feed, item) {
                Ok(item) => channel.items.push(item),
                Err(report) => {
                    let error = report.current_context();
                    if error != &EmulateError::NoPath {
                        return Err(report);
                    }
                    trace!(episode, "Skipping episode as it has not been downloaded");
                }
            }
        }
        if channel.items.is_empty() {
            trace!(season, year, "Skipping feed as it contains no episodes");
            return Ok(None);
        }
        let xml = channel.to_string();
        let path = self.paths.get_rss_path(&feed.podcast.slug, season, year);
        create_parent_dir_if_not_exist(&path)
            .await
            .change_context(EmulateError::CreateDirectory)?;
        let mut file = AsyncFile::create(&path)
            .await
            .change_context(EmulateError::Create)
            .attach_path(&path)?;
        file.write_all(xml.as_bytes())
            .await
            .change_context(EmulateError::Write)
            .attach_path(&path)?;
        file.flush()
            .await
            .change_context(EmulateError::Flush)
            .attach_path(&path)?;
        Ok(Some(path))
    }

    fn replace_enclosure(
        &self,
        feed: &PodcastFeed,
        mut item: RssItem,
    ) -> Result<RssItem, Report<EmulateError>> {
        let guid = item.guid.clone().ok_or(EmulateError::NoGuid)?;
        let episode = feed
            .episodes
            .iter()
            .find(|episode| episode.source_id == guid.value)
            .ok_or(EmulateError::NoMatch)?;
        let Some(enclosure) = item.enclosure.as_mut() else {
            return Err(Report::new(EmulateError::NoEnclosure));
        };
        enclosure.url = self.get_audio_url(episode)?.to_string();
        Ok(item)
    }

    /// URL of the episode audio file.
    ///
    /// If the `server_base` option is not set this falls back to a `file://` URL.
    ///
    /// Examples:
    /// - `https://example.com/irl/S00/1970/1970-01-01 001 Hello World.mp3`
    /// - `file://$HOME/.local/share/alnwick/podcasts/irl/S00/1970/1970-01-01 001 Hello World.mp3`
    fn get_audio_url(&self, episode: &EpisodeInfo) -> Result<Url, Report<EmulateError>> {
        let Some(sub_path) = &episode.file_sub_path else {
            return Err(Report::new(EmulateError::NoPath).attach_episode(episode));
        };
        let Some(base) = &self.options.server_base else {
            return Err(Report::new(EmulateError::NoServerBase));
        };
        base.join(sub_path.to_string_lossy().as_ref())
            .change_context(EmulateError::ParseUrl)
    }
}

fn group_by_season(episodes: Vec<EpisodeInfo>) -> HashMap<Option<u32>, Vec<EpisodeInfo>> {
    let mut groups: HashMap<Option<u32>, Vec<EpisodeInfo>> = HashMap::new();
    for episode in episodes {
        let group = groups.entry(episode.season).or_default();
        group.push(episode);
    }
    groups
}

fn group_by_year(episodes: Vec<EpisodeInfo>) -> HashMap<i32, Vec<EpisodeInfo>> {
    let mut groups: HashMap<i32, Vec<EpisodeInfo>> = HashMap::new();
    for episode in episodes {
        let year = episode.published_at.year();
        let group = groups.entry(year).or_default();
        group.push(episode);
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn emulate_handler() {
        // Arrange
        let services = MockServices::default().create().await;
        let handler = services
            .get_async::<EmulateHandler>()
            .await
            .expect("should be able to get handler");
        let request = EmulateRequest {
            slug: MockFeeds::podcast_slug(),
        };
        let _logger = init_test_logger();

        // Act
        let result = handler.execute(&request).await;

        // Assert
        result.assert_ok_debug();
    }
}
