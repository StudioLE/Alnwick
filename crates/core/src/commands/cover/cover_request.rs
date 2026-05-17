use crate::prelude::*;

/// A request to execute a [`CoverHandler`].
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CoverRequest {
    /// User-defined identifier for the podcast.
    pub slug: Slug,
}

impl Display for CoverRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "cover {}", self.slug)
    }
}

impl Executable for CoverRequest {
    type Response = CoverResponse;
    type ExecutionError = Report<CoverError>;
}
