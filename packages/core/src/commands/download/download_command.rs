use crate::prelude::*;

pub struct DownloadCommand {
    pub(super) paths: Arc<PathProvider>,
    pub(super) http: Arc<HttpClient>,
    pub(super) metadata: Arc<MetadataRepository>,
}

impl Service for DownloadCommand {
    type Error = ServiceError;

    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self::new(
            services.get_service().await?,
            services.get_service().await?,
            services.get_service().await?,
        ))
    }
}

impl DownloadCommand {
    #[must_use]
    pub fn new(
        paths: Arc<PathProvider>,
        http: Arc<HttpClient>,
        metadata: Arc<MetadataRepository>,
    ) -> Self {
        Self {
            paths,
            http,
            metadata,
        }
    }
}

impl Command for DownloadCommand {
    type Input = DownloadRequest;
    type Output = ();
    type CommandError = DownloadError;

    async fn execute(&self, request: DownloadRequest) -> Result<(), Report<DownloadError>> {
        let context = self.context_step(&request).await?;
        let podcast = context.podcast.to_string();
        let episode = context.episode.to_string();
        trace!(podcast, episode, "Downloading episode file");
        self.download_episode_step(&context).await?;
        trace!(podcast, episode, "Downloading episode image");
        self.download_image_step(&context).await?;
        trace!(podcast, episode, "Resizing episode image");
        self.resize_step(&context).await?;
        trace!(podcast, episode, "Tagging episode");
        self.tag_step(&context)?;
        trace!(podcast, episode, "Saving episode");
        self.save_step(&context).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[traced_test]
    pub async fn download_command() {
        // Arrange
        let services = TestServiceProvider::create().await;
        let command = services
            .get_service::<DownloadCommand>()
            .await
            .expect("should be able to get command");
        let request = DownloadRequest::new(PODCAST_KEY, EPISODE_KEY);

        // Act
        let result = command.execute(request).await;

        // Assert
        result.assert_ok_debug();
    }
}
