use crate::prelude::*;

const BANNER_WIDTH: u32 = 960;
const BANNER_HEIGHT: u32 = 540;
const COVER_SIZE: u32 = 720;

/// Download and resize podcast cover and banner images.
#[derive(Clone, FromServicesAsync)]
pub struct CoverHandler {
    paths: Arc<PathProvider>,
    http: Arc<dyn HttpFetch>,
    metadata: Arc<MetadataRepository>,
}

#[async_trait]
impl Execute<CoverRequest, CoverResponse, Report<CoverError>> for CoverHandler {
    /// Execute the cover handler.
    async fn execute(&self, request: &CoverRequest) -> Result<CoverResponse, Report<CoverError>> {
        let feed = self
            .metadata
            .get_feed_by_slug(request.slug.clone(), None)
            .await
            .change_context(CoverError::Repository)?
            .ok_or(CoverError::NoPodcast)?;
        let url = feed.podcast.image.clone().ok_or(CoverError::NoImage)?;
        let cover = self.paths.get_cover_path(&request.slug);
        let temp_path = temp_path(&cover);
        self.http
            .download(&url, temp_path.clone())
            .await
            .change_context(CoverError::GetImage)
            .attach_url(&url)?;
        let banner = self.paths.get_banner_path(&request.slug);
        create_parent_dir_if_not_exist(&banner)
            .await
            .change_context(CoverError::CreateDirectory)?;
        let resize = Resize::new(&temp_path)
            .change_context(CoverError::CreateImage)
            .attach_path(&temp_path)?;
        resize
            .to_file(&banner, BANNER_WIDTH, BANNER_HEIGHT)
            .change_context(CoverError::CreateImage)?;
        resize
            .to_file(&cover, COVER_SIZE, COVER_SIZE)
            .change_context(CoverError::CreateImage)?;
        if let Err(error) = remove_file(&temp_path).await {
            warn!(%error, path = %temp_path.display(), "Failed to remove temp cover file");
        }
        info!("Created images");
        trace!(banner = %banner.display(), cover = %cover.display(), "Created images");
        Ok(CoverResponse {
            banner_path: banner,
            cover_path: cover,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn cover_handler() {
        // Arrange
        let services = MockServices::default().create().await;
        let handler = services
            .get_async::<CoverHandler>()
            .await
            .expect("should be able to get handler");
        let request = CoverRequest {
            slug: MockFeeds::podcast_slug(),
        };
        let _logger = init_test_logger();

        // Act
        let result = handler.execute(&request).await;

        // Assert
        result.assert_ok_debug();
    }
}
