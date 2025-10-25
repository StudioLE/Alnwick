use crate::core::schema::error::PodcastConvertError;
use crate::prelude::*;

#[derive(Debug)]
pub enum ScrapeError {
    Head(HttpError),
    Simplecast(ScrapeSimplecastError),
    Rss(ScrapeRssError),
    Save(DatabaseError),
}

#[derive(Debug)]
#[allow(clippy::absolute_paths)]
pub enum ScrapeRssError {
    Xml(HttpError),
    IO(String, PathBuf, std::io::Error),
    Parse(rss::Error),
    Convert(PodcastConvertError),
}

impl Display for ScrapeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let reason = match self {
            ScrapeError::Head(e) => format!("Unable to get content type:\n{e}"),
            ScrapeError::Simplecast(e) => format!("{e}"),
            ScrapeError::Rss(e) => {
                format!("{e}",)
            }
            ScrapeError::Save(e) => format!("Unable to save: {e}"),
        };
        write!(f, "{} to scrape\n{reason}", "Failed".bold())
    }
}

impl Display for ScrapeRssError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let reason = match self {
            ScrapeRssError::Xml(e) => format!("Unable to get feed:\n{e}"),
            ScrapeRssError::IO(id, path, e) => {
                format!(
                    "An I/O error occurred while processing episode: {id}\nPath: {}\n{e}",
                    path.display()
                )
            }
            ScrapeRssError::Parse(e) => {
                format!("Unable to parse RSS\n{e}",)
            }
            ScrapeRssError::Convert(e) => {
                format!("Unable to convert RSS\n{e}",)
            }
        };
        write!(f, "{} to scrape\n{reason}", "Failed".bold())
    }
}

#[derive(Debug)]
pub enum ScrapeSimplecastError {
    GetPage(HttpError),
    PlayerNotFound(Url),
    GetEpisode(String, HttpError),
    GetPlaylist(String, HttpError),
}

impl Display for ScrapeSimplecastError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let reason = match self {
            ScrapeSimplecastError::GetPage(e) => format!("Unable to get page\n{e}"),
            ScrapeSimplecastError::PlayerNotFound(url) => {
                format!("Page does not contain a Simplecast Player\nURL: {url}")
            }
            ScrapeSimplecastError::GetEpisode(id, e) => format!("Unable to get episode: {id}\n{e}"),
            ScrapeSimplecastError::GetPlaylist(id, e) => {
                format!("Unable to get playlist: {id}\n{e}")
            }
        };
        write!(f, "{} to scrape\n{reason}", "Failed".bold())
    }
}
