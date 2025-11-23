use crate::prelude::*;
use sea_orm::*;

impl MetadataRepository {
    /// Get all podcasts with minimal info for the index page.
    pub async fn get_podcasts(&self) -> Result<Vec<IndexPagePodcastPartial>, DbErr> {
        let podcasts = podcast::Entity::find()
            .order_by_asc(podcast::Column::Title)
            .join(JoinType::LeftJoin, podcast::Relation::Episode.def())
            .group_by(podcast::Column::PrimaryKey)
            .into_partial_model::<IndexPagePodcastPartial>()
            .all(&self.db)
            .await?;
        Ok(podcasts)
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[tokio::test]
    #[traced_test]
    pub async fn get_podcasts() {
        // Arrange
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");

        // Act
        let result = services.metadata.get_podcasts().await;

        // Assert
        let podcasts = result.assert_ok_debug();
        for podcast in podcasts {
            println!(
                "{} · {} · {}",
                podcast.slug, podcast.episodes, podcast.title
            );
            assert!(podcast.episodes > 0);
        }
    }
}
