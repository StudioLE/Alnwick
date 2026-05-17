use crate::prelude::*;

/// CLI command for downloading and resizing podcast cover images.
#[derive(FromServicesAsync)]
pub struct CoverCliCommand {
    selector: Arc<PodcastSelector>,
    cli_runner: Arc<CliRunner>,
}

impl CoverCliCommand {
    /// Download and resize cover images for podcasts matching the options.
    pub async fn execute(
        &self,
        options: PodcastOptions,
    ) -> Result<(), Report<PodcastSelectorError>> {
        let slugs = self.selector.execute(&options).await?;
        let requests = slugs.into_iter().map(|slug| CoverRequest { slug });
        let status = self.cli_runner.run(requests).await;
        for (_request, error) in &status.failed {
            warn!("{}", error.render());
        }
        info!("Processed covers for {} podcasts", status.succeeded.len());
        if !status.failed.is_empty() {
            warn!(
                "Failed to process covers for {} podcasts",
                status.failed.len()
            );
        }
        Ok(())
    }
}
