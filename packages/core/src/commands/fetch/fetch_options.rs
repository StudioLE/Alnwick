use crate::prelude::*;

/// CLI options for [`FetchCliCommand`].
#[derive(Clone, Debug, Args)]
pub struct FetchOptions {
    /// Slug of the podcast to fetch.
    ///
    /// Must be alphanumeric and hyphenated.
    pub podcast_slug: Slug,
}

impl From<FetchOptions> for FetchRequest {
    fn from(options: FetchOptions) -> Self {
        Self {
            slug: options.podcast_slug,
        }
    }
}
