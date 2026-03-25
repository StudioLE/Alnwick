use crate::prelude::*;

pub struct MountProvider;

impl FromServices for MountProvider {
    type Error = ResolveError;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Err(Report::new(ResolveError::NotFound))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_service_none() {
        // Arrange
        let services = ServiceBuilder::new().with_core().build();
        let _logger = init_test_logger();

        // Act
        let result = services.get_async::<MountProvider>().await;

        // Assert
        assert!(result.is_err());
    }
}
