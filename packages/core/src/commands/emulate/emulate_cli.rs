use crate::prelude::*;

/// Maximum concurrent emulations.
const CONCURRENCY: usize = 8;

/// CLI command for generating emulated RSS feeds.
///
/// Queues multiple [`EmulateRequest`] and executes them concurrently
/// with a progress bar.
#[derive(FromServicesAsync)]
pub struct EmulateCliCommand {
    selector: Arc<PodcastSelector>,
    runner: Arc<CommandRunner<CommandInfo>>,
    progress: Arc<CliProgress<CommandInfo>>,
}

impl EmulateCliCommand {
    /// Generate emulated RSS feeds for podcasts matching the options.
    pub async fn execute(
        &self,
        options: PodcastOptions,
    ) -> Result<(), Report<PodcastSelectorError>> {
        let slugs = self.selector.execute(&options).await?;
        trace!(count = slugs.len(), "Emulating podcasts");
        self.progress.start().await;
        for slug in slugs {
            let request = EmulateRequest { slug };
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
                CommandStatus::Succeeded(CommandSuccess::Emulate(_)) => succeeded += 1,
                CommandStatus::Failed(CommandFailure::Emulate(e)) => {
                    failed += 1;
                    warn!("{}", e.render());
                }
                _ => unreachable!("should only get emulate results"),
            }
        }
        info!("Emulated {succeeded} podcasts");
        if failed > 0 {
            warn!("Failed to emulate {failed} podcasts");
        }
        Ok(())
    }
}
