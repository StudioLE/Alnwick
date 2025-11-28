use crate::prelude::*;

/// A request to execute a [`DownloadCommand`].
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct DownloadRequest {
    pub(super) podcast: PodcastKey,
    pub(super) episode: EpisodeKey,
}

impl DownloadRequest {
    #[must_use]
    pub fn new(podcast: PodcastKey, episode: EpisodeKey) -> Self {
        Self { podcast, episode }
    }
}
