use crate::prelude::*;

/// CLI options for [`AddCliCommand`].
#[derive(Clone, Debug, Args)]
pub struct AddOptions {
    /// Slug for the podcast.
    ///
    /// Must be alphanumeric and hyphenated.
    pub slug: Slug,
    /// URL of the RSS feed.
    pub feed_url: UrlWrapper,
}

impl From<AddOptions> for AddRequest {
    fn from(options: AddOptions) -> Self {
        Self {
            slug: options.slug,
            feed_url: options.feed_url,
        }
    }
}
