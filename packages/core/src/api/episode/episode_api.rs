use crate::prelude::*;
use sea_orm::*;

impl MetadataRepository {
    /// Get an episode with minimal info for the episode page.
    pub async fn get_episode(
        &self,
        podcast_slug: &str,
        episode_key: u32,
    ) -> Result<Option<EpisodePagePartial>, DbErr> {
        episode::Entity::find_by_id(episode_key)
            .join(JoinType::InnerJoin, episode::Relation::Podcast.def())
            .filter(podcast::Column::Slug.eq(podcast_slug))
            .into_partial_model::<EpisodePagePartial>()
            .one(&self.db)
            .await
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[tokio::test]
    #[traced_test]
    pub async fn get_episode() {
        // Arrange
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");

        // Act
        let result = services.metadata.get_episode("irl", 1).await;

        // Assert
        let episode = result.assert_ok_debug().expect("Episode should exist");
        assert_eq!(episode.primary_key, 1);
    }
}
