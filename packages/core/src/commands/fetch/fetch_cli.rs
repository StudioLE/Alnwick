use crate::prelude::*;

/// CLI command for fetching existing podcasts.
#[derive(FromServicesAsync)]
pub struct FetchCliCommand {
    selector: Arc<PodcastSelector>,
    cli_runner: Arc<CliRunner>,
}

impl FetchCliCommand {
    /// Fetch podcasts matching the options.
    pub async fn execute(
        &self,
        options: PodcastOptions,
    ) -> Result<(), Report<PodcastSelectorError>> {
        let slugs = self.selector.execute(&options).await?;
        let requests = slugs.into_iter().map(|slug| FetchRequest { slug });
        let status = self.cli_runner.run(requests).await;
        for (_request, error) in &status.failed {
            warn!("{}", error.render());
        }
        info!("Fetched {} podcasts", status.succeeded.len());
        if !status.failed.is_empty() {
            warn!("Failed to fetch {} podcasts", status.failed.len());
        }
        Ok(())
    }
}
