use crate::prelude::*;

#[derive(Args, Debug, Default, Serialize)]
pub struct FilterOptions {
    /// Only include episodes with the specified season
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<usize>,
    /// Exclude episodes before the specified season
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_season: Option<usize>,
    /// Exclude episodes after the specified season
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_season: Option<usize>,
    /// Only include episodes with the specified year
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    /// Exclude episodes before the specified year
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_year: Option<i32>,
    /// Exclude episodes after the specified year
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_year: Option<i32>,
}

impl Display for FilterOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let yaml = serde_yaml::to_string(self).expect("should be able to serialize");
        let output = yaml.trim_end().replace("\r\n", " ").replace('\n', " ");
        write!(f, "{output}")
    }
}

impl Podcast {
    pub(crate) fn filter(&mut self, options: &FilterOptions) {
        let before = self.episodes.len();
        self.episodes.retain(|episode| {
            if let Some(year) = options.year {
                if episode.published_at.year() != year {
                    return false;
                }
            }
            if let Some(year) = options.from_year {
                if episode.published_at.year() < year {
                    return false;
                }
            }
            if let Some(year) = options.to_year {
                if episode.published_at.year() > year {
                    return false;
                }
            }
            if let Some(season) = options.season {
                let Some(episode_season) = episode.season else {
                    return false;
                };
                if episode_season != season {
                    return false;
                }
            }
            if let Some(season) = options.from_season {
                let Some(episode_season) = episode.season else {
                    return false;
                };
                if episode_season < season {
                    return false;
                }
            }
            if let Some(season) = options.to_season {
                let Some(episode_season) = episode.season else {
                    return false;
                };
                if episode_season > season {
                    return false;
                }
            }
            true
        });
        let after = self.episodes.len();
        debug!("{} includes {after} of {before} episodes", "Filter".bold());
        trace!("Options: {options}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn filter() {
        // Arrange
        let _ = init_logging();
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");
        let podcast = services.metadata.get("irl").expect("podcast should exist");

        // Act
        let mut y2019 = podcast.clone();
        y2019.filter(&FilterOptions {
            year: Some(2019),
            ..FilterOptions::default()
        });
        let mut y2019_2020 = podcast.clone();
        y2019_2020.filter(&FilterOptions {
            from_year: Some(2019),
            to_year: Some(2020),
            ..FilterOptions::default()
        });
        let mut s1 = podcast.clone();
        s1.filter(&FilterOptions {
            season: Some(1),
            ..FilterOptions::default()
        });
        let mut s1_2 = podcast.clone();
        s1_2.filter(&FilterOptions {
            from_season: Some(1),
            to_season: Some(2),
            ..FilterOptions::default()
        });
        let mut s1 = podcast.clone();
        s1.filter(&FilterOptions {
            season: Some(1),
            ..FilterOptions::default()
        });
        let mut s4_2018 = podcast.clone();
        s4_2018.filter(&FilterOptions {
            season: Some(4),
            year: Some(2018),
            ..FilterOptions::default()
        });

        // Assert
        assert!(podcast.episodes.len() >= 60);
        assert_eq!(y2019.episodes.len(), 13);
        assert_eq!(y2019_2020.episodes.len(), 13);
        assert_eq!(s1.episodes.len(), 10);
        assert_eq!(s1_2.episodes.len(), 17);
        assert_eq!(s4_2018.episodes.len(), 3);
    }
}
