use crate::prelude::Error;

#[derive(Clone, Debug, Error)]
pub enum ListError {
    #[error("Unable to get podcasts")]
    GetPodcast,
    #[error("Unable to read directory")]
    ReadDirectory,
    #[error("Unable to read directory entry")]
    ReadEntry,
}
