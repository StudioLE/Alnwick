use crate::prelude::*;

/// Shared CLI arguments for selecting which podcasts to operate on.
///
/// Requires either `--podcast <SLUG>` or `--all-podcasts`.
#[derive(Clone, Debug, Args)]
#[command(group = clap::ArgGroup::new("target").required(true).multiple(false))]
pub struct PodcastOptions {
    /// Slug of the podcast.
    #[arg(long, group = "target")]
    pub podcast: Option<Slug>,
    /// Run against all podcasts.
    #[arg(long, group = "target")]
    pub all_podcasts: bool,
}
