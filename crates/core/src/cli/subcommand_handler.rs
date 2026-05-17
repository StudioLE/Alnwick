//! Subcommand dispatch handler.

use crate::prelude::*;

/// Handler that dispatches the selected [`CliSubcommand`].
#[derive(FromServicesAsync)]
pub struct SubcommandHandler {
    cli_args: Arc<CliArgs>,
    add: Arc<AddCliCommand>,
    fetch: Arc<FetchCliCommand>,
    download: Arc<DownloadCliCommand>,
    emulate: Arc<EmulateCliCommand>,
    cover: Arc<CoverCliCommand>,
}

impl SubcommandHandler {
    /// Execute the selected subcommand.
    pub async fn run(&self) -> Result<(), StructuredError> {
        let command = self.cli_args.command.clone();
        match command {
            CliSubcommand::Add(options) => {
                self.add.execute(options).await?;
            }
            CliSubcommand::Fetch(options) => {
                self.fetch.execute(options).await?;
            }
            CliSubcommand::Download(options) => {
                self.download.execute(options).await?;
            }
            CliSubcommand::Emulate(options) => {
                self.emulate.execute(options).await?;
            }
            CliSubcommand::Cover(options) => {
                self.cover.execute(options).await?;
            }
        }
        Ok(())
    }
}
