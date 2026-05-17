use crate::prelude::*;

/// A request to execute an [`AddHandler`].
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AddRequest {
    /// User-defined identifier for the podcast.
    pub slug: Slug,
    /// URL of the RSS feed.
    pub feed_url: UrlWrapper,
}

impl Display for AddRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Slug: {} URL: {}", self.slug, self.feed_url)
    }
}

impl Executable for AddRequest {
    type Response = AddResponse;
    type ExecutionError = Report<AddError>;
}
