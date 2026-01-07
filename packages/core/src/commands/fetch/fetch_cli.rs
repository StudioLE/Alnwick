use crate::prelude::*;

/// CLI command for fetching an existing podcast.
pub struct FetchCliCommand {
    handler: Arc<FetchHandler>,
}

impl Service for FetchCliCommand {
    type Error = ServiceError;

    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self {
            handler: services.get_service().await?,
        })
    }
}

impl FetchCliCommand {
    /// Execute the fetch command.
    pub async fn execute(
        &self,
        options: FetchOptions,
    ) -> Result<FetchResponse, Report<FetchError>> {
        self.handler.execute(&FetchRequest::from(options)).await
    }
}
