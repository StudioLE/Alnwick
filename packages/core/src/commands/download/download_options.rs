use crate::prelude::*;

/// CLI options for batch downloading episodes from a podcast.
#[derive(Debug, Args)]
pub struct DownloadOptions {
    /// Podcast selection options.
    #[command(flatten)]
    pub selection: PodcastOptions,
    #[command(flatten)]
    pub filter: FilterOptions,
    /// Replace existing downloads by re-downloading and re-processing.
    #[arg(long)]
    pub replace: bool,
}
