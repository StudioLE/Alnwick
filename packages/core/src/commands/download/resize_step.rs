use crate::prelude::*;
use tokio::task::spawn_blocking;

/// Target dimension for resized artwork in pixels.
const IMAGE_SIZE: u32 = 720;

impl DownloadHandler {
    /// Resize episode artwork.
    ///
    /// - Resizes to a square with dimensions defined by [`IMAGE_SIZE`]
    /// - Runs in a blocking task via [`spawn_blocking`]
    /// - Saves to file
    pub(super) async fn resize_step(
        &self,
        context: &DownloadContext,
    ) -> Result<(), Report<DownloadError>> {
        let Some(path) = context.image_path.clone() else {
            return Ok(());
        };
        spawn_blocking(move || -> Result<(), Report<DownloadError>> {
            let resize = Resize::new(&path)
                .attach_path(&path)
                .change_context(DownloadError::ReadImage)?;
            resize
                .to_file(&path, IMAGE_SIZE, IMAGE_SIZE)
                .change_context(DownloadError::ResizeImage)
        })
        .await
        .change_context(DownloadError::ResizeTask)?
    }
}
