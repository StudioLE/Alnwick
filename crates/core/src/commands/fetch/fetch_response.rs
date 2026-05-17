use crate::prelude::*;

/// A response returned by [`FetchHandler`].
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct FetchResponse {
    /// Database key of the fetched podcast.
    pub podcast_key: PodcastKey,
    /// Slug of the fetched podcast.
    pub podcast_slug: Slug,
    /// Number of episodes updated.
    pub episodes_updated: usize,
    /// Number of episodes inserted.
    pub episodes_inserted: usize,
}
