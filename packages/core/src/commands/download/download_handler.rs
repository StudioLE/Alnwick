use crate::prelude::*;

/// Downloads a single episode through a multi-step pipeline.
///
/// - Fetch audio file
/// - Fetch and resize artwork
/// - Add ID3 tags
/// - Save file paths to database
#[derive(FromServicesAsync)]
pub struct DownloadHandler {
    pub(super) paths: Arc<PathProvider>,
    pub(super) http: Arc<HttpClient>,
    pub(super) metadata: Arc<MetadataRepository>,
}

#[async_trait]
impl Execute<DownloadRequest, DownloadResponse, Report<DownloadError>> for DownloadHandler {
    /// Execute the download pipeline for a single episode.
    async fn execute(
        &self,
        request: &DownloadRequest,
    ) -> Result<DownloadResponse, Report<DownloadError>> {
        trace!(%request, "Retrieving podcast and episode from DB");
        let context = self.context_step(request).await?;
        let podcast = context.podcast.to_string();
        let episode = context.episode.to_string();
        if let Some(path) = &context.episode.file_sub_path {
            if request.replace {
                debug!(podcast, episode, %path, "Replacing existing download");
                self.delete_existing_step(&context).await;
            } else {
                debug!(podcast, episode, %path, "Skipping already downloaded");
                return Ok(DownloadResponse {
                    file_path: path.as_ref().clone(),
                    image_path: context.episode.image_sub_path.as_deref().cloned(),
                });
            }
        }
        trace!(podcast, episode, "Downloading episode file");
        self.download_file_step(&context).await?;
        trace!(podcast, episode, "Downloading episode image");
        self.download_image_step(&context).await?;
        trace!(podcast, episode, "Resizing episode image");
        self.resize_step(&context).await?;
        trace!(podcast, episode, "Tagging episode");
        self.tag_step(&context)?;
        trace!(podcast, episode, "Saving episode");
        self.save_step(&context).await
    }
}

#[cfg(test)]
mod tests {
    use tokio::fs::{create_dir_all, read, remove_file, write};

    use super::*;

    const MARKER: &[u8] = b"marker";
    const OLD_SUB_PATH: &str = "old-convention/old-file.mp3";

    #[tokio::test]
    #[serial]
    pub async fn download_handler() {
        // Arrange
        let services = MockServices::default().create().await;
        let download = services
            .get_async::<DownloadHandler>()
            .await
            .expect("should be able to get command");
        let request = DownloadRequest::new(MockFeeds::PODCAST_KEY, MockFeeds::EPISODE_KEY, false);
        let _logger = init_test_logger();

        // Act
        let result = download.execute(&request).await;

        // Assert
        result.assert_ok_debug();
    }

    #[tokio::test]
    #[serial]
    pub async fn download_handler_replace() {
        // Arrange
        let services = MockServices::default().create().await;
        let download = services
            .get_async::<DownloadHandler>()
            .await
            .expect("should be able to get command");
        let paths = services
            .get_async::<PathProvider>()
            .await
            .expect("should be able to get path provider");
        let _logger = init_test_logger();

        // Act: first download
        let request = DownloadRequest::new(MockFeeds::PODCAST_KEY, MockFeeds::EPISODE_KEY, false);
        let first_response = download.execute(&request).await.assert_ok_debug();
        let full_path = paths.get_podcasts_dir().join(&first_response.file_path);
        assert!(full_path.exists(), "Downloaded file should exist");

        // Delete and recreate the file with a marker to prove it gets replaced.
        // Must delete first to break any hard link to the cache.
        remove_file(&full_path).await.expect("should remove file");
        write(&full_path, MARKER)
            .await
            .expect("should write marker");
        let before = read(&full_path).await.expect("should read marker");
        assert_eq!(before, MARKER);

        // Act: replace download
        let request = DownloadRequest::new(MockFeeds::PODCAST_KEY, MockFeeds::EPISODE_KEY, true);
        let second_response = download.execute(&request).await.assert_ok_debug();

        // Assert: file was rewritten (no longer contains marker)
        let after = read(&full_path).await.expect("should read replaced file");
        assert_ne!(after, MARKER, "File should be replaced, not the marker");
        assert_eq!(first_response.file_path, second_response.file_path);
    }

    #[tokio::test]
    #[serial]
    pub async fn download_handler_replace_different_path() {
        // Arrange
        let services = MockServices::new()
            .with_metadata_factory(MockFeedsFactory {
                edit_episode: Some(|episode| {
                    episode.file_sub_path =
                        Some(PathWrapper::from_str(OLD_SUB_PATH).expect("should be a valid path"));
                }),
                ..MockFeedsFactory::default()
            })
            .create()
            .await;
        let download = services
            .get_async::<DownloadHandler>()
            .await
            .expect("should be able to get command");
        let paths = services
            .get_async::<PathProvider>()
            .await
            .expect("should be able to get path provider");
        let old_full_path = paths.get_podcasts_dir().join(OLD_SUB_PATH);
        create_dir_all(old_full_path.parent().expect("should have parent"))
            .await
            .expect("should create dir");
        write(&old_full_path, "")
            .await
            .expect("should write fake file");
        let request = DownloadRequest::new(MockFeeds::PODCAST_KEY, MockFeeds::EPISODE_KEY, true);
        let _logger = init_test_logger();

        // Act
        let result = download.execute(&request).await;

        // Assert
        let response = result.assert_ok_debug();
        assert!(!old_full_path.exists(), "Old file should be deleted");
        assert_ne!(response.file_path, PathBuf::from(OLD_SUB_PATH));
    }
}
