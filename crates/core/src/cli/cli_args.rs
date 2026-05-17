//! Parsed CLI arguments registered in the service container.

use crate::prelude::*;
use clap::Subcommand as ClapSubcommand;

/// Parsed CLI arguments.
#[derive(Debug, Parser)]
#[command(name = "alnwick", about = "Self-hosted podcast library")]
pub struct CliArgs {
    /// Set the log level.
    #[arg(long)]
    pub log_level: Option<LogLevel>,
    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: CliSubcommand,
}

impl FromServices for CliArgs {
    type Error = Infallible;

    fn from_services(_: &ServiceProvider) -> Result<Self, Report<Self::Error>>
    where
        Self: Sized,
    {
        Ok(CliArgs::parse())
    }
}

/// Available CLI subcommands.
#[derive(Clone, Debug, ClapSubcommand)]
pub enum CliSubcommand {
    /// Add a new podcast from an RSS feed.
    Add(AddOptions),
    /// Fetch an existing podcast using its stored feed URL.
    Fetch(PodcastOptions),
    /// Download episodes of a podcast.
    Download(DownloadOptions),
    /// Create emulated RSS of a podcast.
    Emulate(PodcastOptions),
    /// Download cover and banner images of a podcast.
    Cover(PodcastOptions),
}
