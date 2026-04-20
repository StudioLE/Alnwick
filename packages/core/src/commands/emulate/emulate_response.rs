use crate::prelude::*;

/// A response returned by [`EmulateHandler`].
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EmulateResponse {
    /// Number of RSS feed files created.
    pub feed_count: usize,
}
