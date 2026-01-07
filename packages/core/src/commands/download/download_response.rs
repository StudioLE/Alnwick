use crate::prelude::*;

/// A response returned by [`DownloadHandler`].
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct DownloadResponse {
    pub file_path: PathBuf,
    pub image_path: Option<PathBuf>,
}

impl DownloadResponse {
    /// Create a new [`DownloadResponse`] from file paths.
    #[must_use]
    pub fn new(file_path: PathBuf, image_path: Option<PathBuf>) -> Self {
        Self {
            file_path,
            image_path,
        }
    }
}
