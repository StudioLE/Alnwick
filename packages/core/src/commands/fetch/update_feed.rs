use crate::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::*;

impl MetadataRepository {
    /// Update an existing podcast feed.
    ///
    /// - Fails if the podcast doesn't exist
    /// - Updates existing episodes by matching `source_id`, preserving download paths
    /// - Inserts new episodes
    /// - Keeps episodes removed from feed (preserves downloaded content)
    pub async fn update_feed(
        &self,
        feed: PodcastFeed,
    ) -> Result<FetchResponse, Report<UpdateError>> {
        let slug = feed.podcast.slug.clone();
        trace!(podcast = %slug, "Updating podcast and merging episodes");
        let tx = self.db.begin().await.change_context(UpdateError::Begin)?;
        let key = get_podcast_key_by_slug(&tx, &feed.podcast.slug)
            .await
            .change_context(UpdateError::CheckExists)?
            .ok_or(UpdateError::NotFound)?;
        let existing = get_existing_episodes(&tx, key)
            .await
            .change_context(UpdateError::GetExistingEpisodes)?;
        update_podcast(&tx, feed.podcast, key)
            .await
            .change_context(UpdateError::Podcast)?;
        let (to_update, to_insert): (Vec<_>, Vec<_>) = feed
            .episodes
            .into_iter()
            .partition(|ep| existing.contains_key(&ep.source_id));
        let response = FetchResponse {
            podcast_key: key,
            podcast_slug: slug,
            episodes_inserted: to_insert.len(),
            episodes_updated: to_update.len(),
        };
        for episode in to_update {
            let existing_key = existing
                .get(&episode.source_id)
                .copied()
                .expect("partition guarantees existence");
            update_episode_query(episode, existing_key, key)
                .exec(&tx)
                .await
                .change_context(UpdateError::Episodes)?;
        }
        if !to_insert.is_empty() {
            insert_episodes(to_insert, key)
                .exec(&tx)
                .await
                .change_context(UpdateError::Episodes)?;
        }
        tx.commit().await.change_context(UpdateError::Commit)?;
        Ok(response)
    }
}

fn update_podcast_query(
    podcast: PodcastInfo,
    primary_key: PodcastKey,
) -> UpdateOne<podcast::ActiveModel> {
    let model = podcast::ActiveModel {
        primary_key: Unchanged(primary_key),
        slug: Unchanged(podcast.slug),
        feed_url: Set(podcast.feed_url),
        title: Set(podcast.title),
        description: Set(podcast.description),
        image: Set(podcast.image),
        language: Set(podcast.language),
        categories: Set(podcast.categories),
        explicit: Set(podcast.explicit),
        author: Set(podcast.author),
        link: Set(podcast.link),
        kind: Set(podcast.kind),
        copyright: Set(podcast.copyright),
        new_feed_url: Set(podcast.new_feed_url),
        generator: Set(podcast.generator),
    };
    podcast::Entity::update(model)
}

async fn update_podcast(
    tx: &DatabaseTransaction,
    podcast: PodcastInfo,
    primary_key: PodcastKey,
) -> Result<podcast::Model, DbErr> {
    update_podcast_query(podcast, primary_key).exec(tx).await
}

fn get_existing_episodes_query(podcast_key: PodcastKey) -> Select<episode::Entity> {
    episode::Entity::find()
        .select_only()
        .columns([episode::Column::SourceId, episode::Column::PrimaryKey])
        .filter(episode::Column::PodcastKey.eq(podcast_key))
}

async fn get_existing_episodes(
    tx: &DatabaseTransaction,
    podcast_key: PodcastKey,
) -> Result<HashMap<String, EpisodeKey>, DbErr> {
    let hash_map = get_existing_episodes_query(podcast_key)
        .into_tuple()
        .all(tx)
        .await?
        .into_iter()
        .collect();
    Ok(hash_map)
}

fn update_episode_query(
    episode: EpisodeInfo,
    existing_key: EpisodeKey,
    podcast_key: PodcastKey,
) -> UpdateOne<episode::ActiveModel> {
    let model = episode::ActiveModel {
        primary_key: Unchanged(existing_key),
        podcast_key: Unchanged(Some(podcast_key)),
        file_sub_path: Unchanged(None),
        image_sub_path: Unchanged(None),
        source_id: Set(episode.source_id),
        title: Set(episode.title),
        source_url: Set(episode.source_url),
        source_file_size: Set(episode.source_file_size),
        source_content_type: Set(episode.source_content_type),
        published_at: Set(episode.published_at),
        description: Set(episode.description),
        source_duration: Set(episode.source_duration),
        image: Set(episode.image),
        explicit: Set(episode.explicit),
        itunes_title: Set(episode.itunes_title),
        episode: Set(episode.episode),
        season: Set(episode.season),
        kind: Set(episode.kind),
    };
    episode::Entity::update(model)
}

fn get_podcast_key_by_slug_select(slug: &Slug) -> Select<podcast::Entity> {
    podcast::Entity::find()
        .select_only()
        .column(podcast::Column::PrimaryKey)
        .filter(podcast::Column::Slug.eq(slug.as_str()))
}

async fn get_podcast_key_by_slug(
    tx: &DatabaseTransaction,
    slug: &Slug,
) -> Result<Option<u32>, DbErr> {
    get_podcast_key_by_slug_select(slug)
        .into_tuple::<u32>()
        .one(tx)
        .await
}

/// Errors from [`MetadataRepository::update_feed`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum UpdateError {
    #[error("Unable to begin database transaction")]
    Begin,
    #[error("Unable to check if podcast exists")]
    CheckExists,
    #[error("Podcast with this slug does not exist")]
    NotFound,
    #[error("Unable to get existing episodes")]
    GetExistingEpisodes,
    #[error("Unable to update podcast")]
    Podcast,
    #[error("Unable to update or insert episodes")]
    Episodes,
    #[error("Unable to commit database transaction")]
    Commit,
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn _get_podcast_key_by_slug_select() {
        // Arrange
        let slug = MockFeeds::podcast_slug();

        // Act
        let statement = get_podcast_key_by_slug_select(&slug).build(DB_BACKEND);

        // Assert
        assert_snapshot!(format_sql(&statement));
    }

    #[test]
    fn _update_podcast_query() {
        // Arrange
        let feed = MockFeeds::default()
            .feeds
            .into_iter()
            .next()
            .expect("should have at least one feed");

        // Act
        let statement = update_podcast_query(feed.podcast, MockFeeds::PODCAST_KEY)
            .validate()
            .expect("query should be valid")
            .build(DB_BACKEND);

        // Assert
        assert_snapshot!(format_sql(&statement));
    }

    #[test]
    fn _get_existing_episodes_query() {
        // Arrange
        // Act
        let statement = get_existing_episodes_query(MockFeeds::PODCAST_KEY).build(DB_BACKEND);

        // Assert
        assert_snapshot!(format_sql(&statement));
    }

    #[test]
    fn _update_episode_query() {
        // Arrange
        let episode = EpisodeInfo::example();

        // Act
        let statement = update_episode_query(episode, 42, MockFeeds::PODCAST_KEY)
            .validate()
            .expect("query should be valid")
            .build(DB_BACKEND);

        // Assert
        assert_snapshot!(format_sql(&statement));
    }

    #[tokio::test]
    pub async fn update_feed() {
        // Arrange
        let metadata = MockServices::default()
            .create()
            .await
            .get_service::<MetadataRepository>()
            .await
            .expect("should be able to get metadata repository");
        let mut feed = MockFeeds::default()
            .feeds
            .into_iter()
            .next()
            .expect("should have at least one feed");
        let existing_count = feed.episodes.len();
        feed.episodes.push(EpisodeInfo {
            source_id: "new-episode-source-id".to_owned(),
            ..EpisodeInfo::example()
        });
        let _logger = init_test_logger();

        // Act
        let result = metadata.update_feed(feed).await;

        // Assert
        let response = result.assert_ok_debug();
        assert_eq!(response.episodes_updated, existing_count);
        assert_eq!(response.episodes_inserted, 1);
    }

    #[tokio::test]
    pub async fn update_feed__not_found() {
        // Arrange
        let metadata = MockServices::default()
            .create()
            .await
            .get_service::<MetadataRepository>()
            .await
            .expect("should be able to get metadata repository");
        let mut feed = MockFeeds::default()
            .feeds
            .into_iter()
            .next()
            .expect("should have at least one feed");
        feed.podcast.slug = Slug::from_str("non-existent").expect("valid slug");
        let _logger = init_test_logger();

        // Act
        let result = metadata.update_feed(feed).await;

        // Assert
        let err = result.expect_err("should fail when slug doesn't exist");
        assert_eq!(err.current_context(), &UpdateError::NotFound);
    }
}
