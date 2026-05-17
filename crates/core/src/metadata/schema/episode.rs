use crate::prelude::*;
use chrono::DateTime;
use sea_orm::entity::prelude::*;

/// Information about a podcast episode
///
/// - <https://help.apple.com/itc/podcasts_connect/#/itcb54353390>
/// - <https://github.com/Podcastindex-org/podcast-namespace>
pub type EpisodeInfo = Model;

/// `SeaORM` Entity for [`EpisodeInfo`]
#[allow(clippy::struct_field_names)]
#[sea_orm::model]
#[derive(Clone, Debug, DeriveEntityModel, Deserialize, PartialEq, Serialize)]
#[sea_orm(table_name = "episodes")]
pub struct Model {
    // Database
    /// Primary key
    ///
    /// This is auto-incremented by the database
    #[sea_orm(primary_key)]
    pub primary_key: EpisodeKey,

    pub podcast_key: Option<PodcastKey>,

    #[sea_orm(belongs_to, from = "podcast_key", to = "primary_key")]
    pub podcast: HasOne<podcast::Entity>,

    // App
    /// Relative file path to the downloaded audio file.
    ///
    /// Value will be `None` until the file is downloaded with [`DownloadContext`].
    pub file_sub_path: Option<PathWrapper>,
    /// Relative file path to the downloaded image file.
    ///
    /// Value will be `None` until the file is downloaded with [`DownloadContext`].
    pub image_sub_path: Option<PathWrapper>,

    // Required
    /// GUID or Apple Podcasts Episode ID
    pub source_id: String,
    /// Title
    pub title: String,
    /// URL of source media file including a file extension
    /// - Supported file formats include M4A, MP3, MOV, MP4, M4V, and PDF
    pub source_url: UrlWrapper,
    /// Size of source media file in bytes
    pub source_file_size: FileSize,
    /// Mime type of source media file
    pub source_content_type: String,

    // Recommended
    /// Date and time episode was released
    pub published_at: DateTime<FixedOffset>,
    /// HTML formatted description
    pub description: Option<String>,
    /// Duration in seconds
    pub source_duration: Option<Duration>,
    /// URL of JPEG or PNG artwork
    /// - Min: 1400 x 1400 px
    /// - Max: 3000 x 3000 px
    pub image: Option<UrlWrapper>,
    /// Parental advisory information
    pub explicit: Option<bool>,

    // Situationial
    /// Apple Podcasts specific title
    pub itunes_title: Option<String>,
    /// Episode number
    pub episode: Option<EpisodeNumber>,
    /// Season number
    pub season: Option<SeasonNumber>,
    /// Episode type
    pub kind: Option<EpisodeKind>,
}

impl EpisodeInfo {
    #[must_use]
    pub fn example() -> Self {
        Self {
            primary_key: u32::default(),
            podcast_key: None,
            file_sub_path: None,
            image_sub_path: None,
            title: "Lorem ipsum dolor sit amet".to_owned(),
            source_url: UrlWrapper::from_str("https://example.com/season-1/episode-1.mp3").expect("URL should be valid"),
            source_file_size: 1024,
            source_content_type: "audio/mpeg".to_owned(),
            source_id: "550e8400-e29b-41d4-a716-446655440000".to_owned(),
            published_at: DateTime::default(),
            description: Some("Aenean sit amet sem quis velit viverra vestibulum. Vivamus aliquam mattis ipsum, a dignissim elit pulvinar vitae. Aliquam neque risus, tincidunt sit amet elit quis, malesuada ultrices urna.".to_owned()),
            source_duration: None,
            image: Some(UrlWrapper::from_str("https://example.com/image.jpg").expect("URL should be valid")),
            explicit: None,
            itunes_title: None,
            episode: Some(3),
            season: Some(2),
            kind: Some(EpisodeKind::default()),
        }
    }
}

impl Display for EpisodeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let file_stem = get_episode_file_stem(
            self.title.clone(),
            self.published_at,
            self.season,
            self.episode,
            self.kind,
        );
        write!(f, "{file_stem}")
    }
}

impl ActiveModelBehavior for ActiveModel {}
