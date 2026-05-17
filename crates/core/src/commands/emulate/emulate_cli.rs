use crate::prelude::*;

/// CLI command for generating emulated RSS feeds.
#[derive(FromServicesAsync)]
pub struct EmulateCliCommand {
    selector: Arc<PodcastSelector>,
    cli_runner: Arc<CliRunner>,
}

impl EmulateCliCommand {
    /// Generate emulated RSS feeds for podcasts matching the options.
    pub async fn execute(
        &self,
        options: PodcastOptions,
    ) -> Result<(), Report<PodcastSelectorError>> {
        let slugs = self.selector.execute(&options).await?;
        let requests = slugs.into_iter().map(|slug| EmulateRequest { slug });
        let status = self.cli_runner.run(requests).await;
        for (_request, error) in &status.failed {
            warn!("{}", error.render());
        }
        info!("Emulated {} podcasts", status.succeeded.len());
        if !status.failed.is_empty() {
            warn!("Failed to emulate {} podcasts", status.failed.len());
        }
        Ok(())
    }
}
