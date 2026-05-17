use crate::prelude::*;

/// A response returned by [`CoverHandler`].
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CoverResponse {
    /// Path to the generated banner image.
    pub banner_path: PathBuf,
    /// Path to the generated cover image.
    pub cover_path: PathBuf,
}
