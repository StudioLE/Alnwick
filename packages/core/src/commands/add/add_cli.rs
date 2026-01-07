use crate::prelude::*;

/// CLI command for adding a new podcast.
pub struct AddCliCommand {
    handler: Arc<AddHandler>,
}

impl Service for AddCliCommand {
    type Error = ServiceError;

    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self {
            handler: services.get_service().await?,
        })
    }
}

impl AddCliCommand {
    /// Execute the add command.
    pub async fn execute(&self, options: AddOptions) -> Result<AddResponse, Report<AddError>> {
        self.handler.execute(&AddRequest::from(options)).await
    }
}
