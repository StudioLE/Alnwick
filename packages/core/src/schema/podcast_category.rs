use crate::prelude::*;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PodcastCategory {
    pub category: String,
    pub sub_category: Option<String>,
}
