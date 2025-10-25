use crate::prelude::*;
use super::options::CoverOptions;
use super::error::CoverError;

const BANNER_WIDTH: u32 = 960;
const BANNER_HEIGHT: u32 = 540;
const COVER_SIZE: u32 = 720;

pub struct CoverCommand {
    paths: PathProvider,
    http: HttpClient,
    metadata: MetadataStore,
}

impl CoverCommand {
    #[must_use]
    pub fn new(paths: PathProvider, http: HttpClient, metadata: MetadataStore) -> Self {
        Self {
            paths,
            http,
            metadata,
        }
    }

    pub async fn execute(&self, options: CoverOptions) -> Result<(), CoverError> {
        let podcast = self
            .metadata
            .get(&options.podcast_id)
            .map_err(CoverError::GetPodcast)?;
        let url = podcast.image_url.ok_or(CoverError::NoImage)?;
        let src = self
            .http
            .get(&url, None)
            .await
            .map_err(CoverError::GetImage)?;
        let banner = self.paths.get_banner_path(&options.podcast_id);
        let cover = self.paths.get_cover_path(&options.podcast_id);
        let resize = Resize::new(&src).map_err(CoverError::Image)?;
        let banner = resize
            .to_file(&banner, BANNER_WIDTH, BANNER_HEIGHT)
            .map_err(CoverError::Image)?;
        let cover = resize
            .to_file(&cover, COVER_SIZE, COVER_SIZE)
            .map_err(CoverError::Image)?;
        info!("{} cover and banner images", "Created".bold());
        trace!("{}", banner.display());
        trace!("{}", cover.display());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn cover_command() {
        // Arrange
        let _ = init_logging();
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");
        let command = CoverCommand::new(services.paths, services.http, services.metadata);
        let options = CoverOptions {
            podcast_id: "irl".to_owned(),
        };

        // Act
        let result = command.execute(options).await;

        // Assert
        result.assert_ok();
    }
}
