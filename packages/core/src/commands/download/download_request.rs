use crate::prelude::*;

/// A request to execute a [`DownloadHandler`].
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct DownloadRequest {
    pub podcast: PodcastKey,
    pub episode: EpisodeKey,
}

impl DownloadRequest {
    /// Create a new [`DownloadRequest`] from database keys.
    #[must_use]
    pub fn new(podcast: PodcastKey, episode: EpisodeKey) -> Self {
        Self { podcast, episode }
    }
}

impl Display for DownloadRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Podcast: {} Episode: {}", self.podcast, self.episode)
    }
}

impl Executable for DownloadRequest {
    type Response = DownloadResponse;
    type ExecutionError = Report<DownloadError>;
}
