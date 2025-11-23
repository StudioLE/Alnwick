use crate::prelude::*;
use sea_orm::*;

impl MetadataRepository {
    /// Get a podcast with minimal info for the podcast page.
    pub async fn get_podcast(
        &self,
        slug: &str,
    ) -> Result<Option<(PodcastPagePartial, Vec<PodcastPageEpisodePartial>)>, DbErr> {
        let option = podcast::Entity::find_by_slug(slug)
            .into_partial_model::<PodcastPagePartial>()
            .one(&self.db)
            .await?;
        let Some(podcast) = option else {
            return Ok(None);
        };
        let episodes = episode::Entity::find()
            .has_related(
                podcast::Entity,
                podcast::Column::PrimaryKey.eq(podcast.primary_key),
            )
            .order_by_asc(episode::Column::PublishedAt)
            .into_partial_model::<PodcastPageEpisodePartial>()
            .all(&self.db)
            .await?;
        Ok(Some((podcast, episodes)))
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[tokio::test]
    #[traced_test]
    pub async fn get_podcast() {
        // Arrange
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");

        // Act
        let result = services.metadata.get_podcast("irl").await;

        // Assert
        let (_podcast, episodes) = result.assert_ok_debug().expect("Podcast should exist");
        assert!(!episodes.is_empty());
    }
}
