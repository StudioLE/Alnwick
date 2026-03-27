use tokio::fs::remove_file;

use crate::prelude::*;

impl DownloadHandler {
    /// Delete existing audio and image files before re-downloading.
    ///
    /// Resolves old paths from `episode.file_sub_path` and `episode.image_sub_path`
    /// relative to the podcasts directory. Warns on any failure and continues,
    /// since the download pipeline will overwrite or fail at the next stage anyway.
    pub(super) async fn delete_existing_step(&self, context: &DownloadContext) {
        let podcasts_dir = self.paths.get_podcasts_dir();
        if let Some(file_sub_path) = &context.episode.file_sub_path {
            let full_path = podcasts_dir.join(file_sub_path.as_ref());
            match remove_file(&full_path).await {
                Ok(()) => debug!(?full_path, "Deleted existing audio file"),
                Err(e) => warn!(?full_path, %e, "Failed to delete existing audio file"),
            }
        }
        if let Some(image_sub_path) = &context.episode.image_sub_path {
            let full_path = podcasts_dir.join(image_sub_path.as_ref());
            match remove_file(&full_path).await {
                Ok(()) => debug!(?full_path, "Deleted existing image file"),
                Err(e) => warn!(?full_path, %e, "Failed to delete existing image file"),
            }
        }
    }
}
