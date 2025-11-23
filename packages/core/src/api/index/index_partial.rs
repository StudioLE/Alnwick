use crate::prelude::*;
use sea_orm::*;

/// A minimal subset of [`PodcastInfo`] used for the index page.
#[derive(Clone, Debug, DerivePartialModel, Deserialize, PartialEq, Serialize)]
#[sea_orm(entity = "podcast::Entity")]
pub struct IndexPagePodcastPartial {
    /// Primary key
    ///
    /// This is auto-incremented by the database
    pub primary_key: u32,
    /// User defined slug
    pub slug: String,
    /// Title
    pub title: String,
    /// URL of JPEG or PNG artwork
    /// - Min: 1400 x 1400 px
    /// - Max: 3000 x 3000 px
    pub image: Option<String>,
    /// Episode count
    #[sea_orm(from_expr = "episode::Column::PrimaryKey.count()")]
    pub episodes: i64,
}
