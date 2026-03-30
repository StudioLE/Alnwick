use crate::prelude::*;

impl DownloadHandler {
    /// Download the episode audio file to the local filesystem.
    pub(super) async fn download_file_step(
        &self,
        context: &DownloadContext,
    ) -> Result<(), Report<DownloadError>> {
        self.http
            .download(&context.episode.source_url, context.file_path.clone())
            .await
            .change_context(DownloadError::DownloadEpisode)
    }
}
