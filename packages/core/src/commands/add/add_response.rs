use crate::prelude::*;

/// A response returned by [`AddHandler`].
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AddResponse {
    /// Database key of the added podcast.
    pub podcast_key: PodcastKey,
    /// Number of episodes added.
    pub episode_count: usize,
}
