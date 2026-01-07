use crate::prelude::*;

/// Errors that can occur during [`DownloadHandler`].
#[derive(Clone, Debug, Error)]
pub enum DownloadError {
    #[error("Unable to get podcast")]
    GetPodcast,
    #[error("Unable to get episode")]
    GetEpisode,
    #[error("Podcast does not exist")]
    NoPodcast,
    #[error("Episode does not exist")]
    NoEpisode,
    #[error("Unable to download the episode file")]
    DownloadEpisode,
    #[error("Unable to download the episode image")]
    DownloadImage,
    #[error("Unable to read the episode image")]
    ReadImage,
    #[error("Unable to resize the episode image due to task error")]
    ResizeTask,
    #[error("Unable to resize the episode image")]
    ResizeImage,
    #[error("Unable to open the episode image for tag")]
    OpenPicture,
    #[error("Unable to read the episode image for tag")]
    ReadPicture,
    #[error("Unable to tag the episode file")]
    TagEpisode,
    #[error("Unable to update the database")]
    Save,
}
