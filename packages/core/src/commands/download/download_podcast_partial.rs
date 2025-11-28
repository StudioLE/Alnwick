use crate::prelude::*;
use sea_orm::FromQueryResult;

/// Partial of [`PodcastInfo`]
#[derive(Clone, Debug, FromQueryResult, Deserialize, PartialEq, Serialize)]
pub struct DownloadPodcastPartial {
    /// Primary key
    ///
    /// This is auto-incremented by the database
    pub primary_key: PodcastKey,
    /// User defined slug
    pub slug: Slug,
    /// Title
    pub title: String,
}

impl Display for DownloadPodcastPartial {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.slug)
    }
}
