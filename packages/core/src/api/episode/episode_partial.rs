use crate::prelude::*;
use sea_orm::*;

/// A minimal subset of [`EpisodeInfo`] used for the podcast page.
#[derive(Clone, Debug, DerivePartialModel, Deserialize, PartialEq, Serialize)]
#[sea_orm(entity = "episode::Entity")]
pub struct EpisodePagePartial {
    /// Primary key
    ///
    /// This is auto-incremented by the database
    pub primary_key: u32,
    /// Title
    pub title: String,
    /// Date and time episode was released
    pub published_at: DateTime<FixedOffset>,
    /// HTML formatted description
    pub description: Option<String>,
    /// Duration in seconds
    pub source_duration: Option<u32>,
    /// URL of JPEG or PNG artwork
    /// - Min: 1400 x 1400 px
    /// - Max: 3000 x 3000 px
    pub image: Option<String>,
    /// Episode number
    pub episode: Option<u32>,
    /// Season number
    pub season: Option<u32>,
    /// Episode type
    pub kind: Option<EpisodeKind>,
}
