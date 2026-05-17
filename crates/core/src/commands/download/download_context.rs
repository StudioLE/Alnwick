use crate::prelude::*;

/// Internal state passed between [`DownloadHandler`] pipeline steps.
#[derive(Clone, Debug)]
pub struct DownloadContext {
    /// Podcast metadata for tagging.
    pub(super) podcast: DownloadPodcastPartial,
    /// Episode metadata for tagging and path generation.
    pub(super) episode: DownloadEpisodePartial,
    /// Resolved path for the audio file.
    pub(super) file_path: PathBuf,
    /// Resolved path for artwork. `None` if episode has no image URL.
    pub(super) image_path: Option<PathBuf>,
}

impl DownloadContext {
    /// Create a new [`DownloadContext`] from metadata partials and base directory.
    #[must_use]
    pub fn new(
        podcast: DownloadPodcastPartial,
        episode: DownloadEpisodePartial,
        podcasts_dir: PathBuf,
    ) -> Self {
        let sub_path = get_sub_path(&podcast, &episode);
        let file_path = podcasts_dir.join(&sub_path);
        let image_path = episode.image.clone().map(|image| {
            let extension = image.get_extension().unwrap_or_else(|| "jpg".to_owned());
            file_path.with_extension(extension)
        });
        Self {
            podcast,
            episode,
            file_path,
            image_path,
        }
    }
}

/// Sub path for an episodes's audio file.
///
/// Example: `irl/1970/1970-01-01 001 Hello World.mp3`
fn get_sub_path(podcast: &DownloadPodcastPartial, episode: &DownloadEpisodePartial) -> PathBuf {
    let year = episode.published_at.year().to_string();
    let file_stem = get_episode_file_stem(
        episode.title.clone(),
        episode.published_at,
        episode.season,
        episode.episode,
        None,
    );
    let extension = get_episode_file_extenson(&episode.source_content_type).unwrap_or_default();
    let season = format!("S{:02}", episode.season.unwrap_or(0));
    // TODO: Remove season from sub path
    PathBuf::new()
        .join(podcast.slug.to_string())
        .join(season)
        .join(year)
        .join(format!("{file_stem}.{extension}"))
}
