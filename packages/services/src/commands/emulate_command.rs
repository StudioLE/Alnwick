use crate::prelude::*;

pub struct EmulateCommand {
    paths: PathProvider,
    metadata: MetadataStore,
}

impl EmulateCommand {
    #[must_use]
    pub fn new(paths: PathProvider, metadata: MetadataStore) -> Self {
        Self { paths, metadata }
    }

    pub async fn execute(&self, options: EmulateOptions) -> Result<(), Report<EmulateError>> {
        let podcast = self
            .metadata
            .get(&options.podcast_id)
            .change_context(EmulateError::GetPodcast)?;
        let feeds = self.save_feeds(&podcast).await?;
        info!("Created {} rss feeds", feeds.len());
        Ok(())
    }

    async fn save_feeds(&self, podcast: &Podcast) -> Result<Vec<PathBuf>, Report<EmulateError>> {
        let mut paths = Vec::new();
        paths.push(self.save_feed(podcast, None, None).await?);
        let mut podcast = podcast.clone();
        let groups = group_by_season(take(&mut podcast.episodes));
        for (season, episodes) in groups {
            let mut p = podcast.clone();
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
        podcast: &Podcast,
        season: Option<usize>,
        year: Option<i32>,
    ) -> Result<PathBuf, Report<EmulateError>> {
        let mut channel: RssChannel = podcast.into();
        for item in &mut channel.items {
            self.replace_enclosure(podcast, item);
        }
        let xml = channel.to_string();
        let path = self.paths.get_rss_path(&podcast.id, season, year);
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
        Ok(path)
    }

    fn replace_enclosure(&self, podcast: &Podcast, item: &mut RssItem) -> Option<()> {
        let guid = item.guid.clone()?;
        let episode = podcast
            .episodes
            .iter()
            .find(|episode| episode.id == guid.value)?;
        let enclosure = item.enclosure.as_mut()?;
        enclosure.url = self.paths.get_audio_url(&podcast.id, episode)?.to_string();
        Some(())
    }
}

fn group_by_season(episodes: Vec<Episode>) -> HashMap<Option<usize>, Vec<Episode>> {
    let mut groups: HashMap<Option<usize>, Vec<Episode>> = HashMap::new();
    for episode in episodes {
        let group = groups.entry(episode.season).or_default();
        group.push(episode);
    }
    groups
}

fn group_by_year(episodes: Vec<Episode>) -> HashMap<i32, Vec<Episode>> {
    let mut groups: HashMap<i32, Vec<Episode>> = HashMap::new();
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
    #[traced_test]
    pub async fn feeds_command() {
        // Arrange
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");
        let command = EmulateCommand::new(services.paths, services.metadata);
        let options = EmulateOptions {
            podcast_id: "irl".to_owned(),
        };

        // Act
        let result = command.execute(options).await;

        // Assert
        result.assert_ok_debug();
    }
}
