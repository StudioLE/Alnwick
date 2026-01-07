use crate::prelude::*;

/// A response returned by [`FetchHandler`].
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct FetchResponse {
    /// Database key of the fetched podcast.
    pub podcast_key: PodcastKey,
    /// Number of episodes fetched.
    pub episode_count: usize,
}
