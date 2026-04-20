use crate::prelude::*;

/// A request to execute an [`EmulateHandler`].
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EmulateRequest {
    /// User-defined identifier for the podcast.
    pub slug: Slug,
}

impl Display for EmulateRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "emulate {}", self.slug)
    }
}

impl Executable for EmulateRequest {
    type Response = EmulateResponse;
    type ExecutionError = Report<EmulateError>;
}
