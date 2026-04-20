use crate::prelude::*;

/// Maximum concurrent cover operations.
const CONCURRENCY: usize = 8;

/// CLI command for downloading and resizing podcast cover images.
///
/// Queues multiple [`CoverRequest`] and executes them concurrently
/// with a progress bar.
#[derive(FromServicesAsync)]
pub struct CoverCliCommand {
    selector: Arc<PodcastSelector>,
    runner: Arc<CommandRunner<CommandInfo>>,
    progress: Arc<CliProgress<CommandInfo>>,
}

impl CoverCliCommand {
    /// Download and resize cover images for podcasts matching the options.
    pub async fn execute(
        &self,
        options: PodcastOptions,
    ) -> Result<(), Report<PodcastSelectorError>> {
        let slugs = self.selector.execute(&options).await?;
        trace!(count = slugs.len(), "Processing covers");
        self.progress.start().await;
        for slug in slugs {
            let request = CoverRequest { slug };
            self.runner
                .queue_request(request)
                .await
                .expect("should be able to queue request");
        }
        self.runner.start(CONCURRENCY).await;
        self.runner.drain().await;
        self.progress.finish().await;
        let results = self.runner.get_commands().await;
        let mut succeeded = 0_usize;
        let mut failed = 0_usize;
        for (_request, status) in results.iter() {
            match status {
                CommandStatus::Succeeded(CommandSuccess::Cover(_)) => succeeded += 1,
                CommandStatus::Failed(CommandFailure::Cover(e)) => {
                    failed += 1;
                    warn!("{}", e.render());
                }
                _ => unreachable!("should only get cover results"),
            }
        }
        info!("Processed covers for {succeeded} podcasts");
        if failed > 0 {
            warn!("Failed to process covers for {failed} podcasts");
        }
        Ok(())
    }
}
