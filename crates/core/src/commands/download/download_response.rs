use crate::prelude::*;

/// A response returned by [`DownloadHandler`].
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct DownloadResponse {
    pub file_path: PathBuf,
    pub image_path: Option<PathBuf>,
}
