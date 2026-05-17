use crate::prelude::*;

/// Maximum concurrent command executions.
const CONCURRENCY: usize = 8;

/// Shared CLI command runner for batch operations.
///
/// - Queues requests
/// - Executes concurrently
/// - Shows progress bar
/// - Returns typed results split by success/failure
#[derive(FromServicesAsync)]
pub struct CliRunner {
    runner: Arc<CommandRunner<CommandInfo>>,
    progress: Arc<CliProgress<CommandInfo>>,
}

/// Outcome of a batch command execution.
pub struct RunStatus<R: Executable> {
    /// Requests that completed successfully.
    pub succeeded: Vec<(R, R::Response)>,
    /// Requests that failed.
    pub failed: Vec<(R, R::ExecutionError)>,
}

impl CliRunner {
    /// Execute a batch of requests and return typed results.
    pub async fn run<R>(&self, requests: impl IntoIterator<Item = R>) -> RunStatus<R>
    where
        R: Executable + Into<CommandRequest> + TryFrom<CommandRequest> + Send + Sync + 'static,
        R::Response: TryFrom<CommandSuccess>,
        R::ExecutionError: TryFrom<CommandFailure>,
    {
        self.progress.start().await;
        for request in requests {
            self.runner
                .queue_request(request)
                .await
                .expect("should be able to queue request");
        }
        self.runner.start(CONCURRENCY).await;
        self.runner.drain().await;
        self.progress.finish().await;
        let succeeded = self.runner.take_succeeded::<R>().await;
        let failed = self.runner.take_failed::<R>().await;
        RunStatus { succeeded, failed }
    }
}
