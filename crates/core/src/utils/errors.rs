use crate::prelude::*;
use core::result::Result;

pub trait CustomReportExt<E> {
    fn attach_episode(self, episode: &EpisodeInfo) -> Report<E>;
    fn attach_url(self, url: &UrlWrapper) -> Report<E>;
}

pub trait CustomResultExt<T, E> {
    fn attach_episode(self, episode: &EpisodeInfo) -> Result<T, Report<E>>;
    fn attach_url(self, url: &UrlWrapper) -> Result<T, Report<E>>;
}

impl<E: Error + Send + Sync + 'static> CustomReportExt<E> for Report<E> {
    fn attach_episode(self, episode: &EpisodeInfo) -> Report<E> {
        self.attach("Episode", episode)
    }
    fn attach_url(self, url: &UrlWrapper) -> Report<E> {
        self.attach("URL", url)
    }
}

impl<T, E: Error + Send + Sync + 'static> CustomResultExt<T, E> for Result<T, Report<E>> {
    fn attach_episode(self, episode: &EpisodeInfo) -> Result<T, Report<E>> {
        self.attach_with("Episode", || episode)
    }
    fn attach_url(self, url: &UrlWrapper) -> Result<T, Report<E>> {
        self.attach_with("URL", || url)
    }
}
