use crate::prelude::*;
use sea_orm::FromQueryResult;

/// Partial of [`EpisodeInfo`]
#[derive(Clone, Debug, FromQueryResult, Deserialize, PartialEq, Serialize)]
pub struct DownloadEpisodePartial {
    /// Primary key
    pub primary_key: EpisodeKey,
    /// Title
    pub title: String,
    /// Relative file path to the downloaded audio file.
    ///
    /// Value will be `None` until the file is downloaded with [`DownloadContext`].
    pub file_sub_path: Option<PathWrapper>,
    /// Relative file path to the downloaded image file.
    ///
    /// Value will be `None` until the file is downloaded with [`DownloadContext`].
    pub image_sub_path: Option<PathWrapper>,
    /// URL of source media file including a file extension
    /// - Supported file formats include M4A, MP3, MOV, MP4, M4V, and PDF
    pub source_url: UrlWrapper,
    /// Mime type of source media file
    pub source_content_type: String,
    /// Date and time episode was released
    pub published_at: DateTime<FixedOffset>,
    /// URL of JPEG or PNG artwork
    /// - Min: 1400 x 1400 px
    /// - Max: 3000 x 3000 px
    pub image: Option<UrlWrapper>,
    /// Episode number
    pub episode: Option<EpisodeNumber>,
    /// Season number
    pub season: Option<SeasonNumber>,
}

impl Display for DownloadEpisodePartial {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let file_stem = get_episode_file_stem(
            self.title.clone(),
            self.published_at,
            self.season,
            self.episode,
            None,
        );
        write!(f, "{file_stem}")
    }
}
