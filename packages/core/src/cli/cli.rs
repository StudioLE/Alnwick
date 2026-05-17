//! CLI bootstrapping and service initialization.

use crate::prelude::*;
use std::process::ExitCode;

/// Application entrypoint that bootstraps services and runs a subcommand.
pub struct Cli {
    services: ServiceProvider,
}

impl Cli {
    /// Create a new [`Cli`] with the default service registrations.
    #[must_use]
    pub fn new() -> Self {
        Self {
            services: ServiceBuilder::new()
                .with_type::<CliArgs>()
                .with_core()
                .with_commands()
                .build()
                .expect_init(),
        }
    }

    /// Run the CLI to completion, returning the appropriate exit code.
    pub async fn run(&self) -> ExitCode {
        if let Err(e) = self.run_subcommand().await {
            error!("{}", e.render());
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    }

    async fn run_subcommand(&self) -> Result<(), StructuredError> {
        let handler = self.services.get_async::<SubcommandHandler>().await?;
        handler.run().await
    }
}
