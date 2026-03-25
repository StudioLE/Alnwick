use crate::metadata::migration::migration_di::PATH_PROVIDER;
use crate::prelude::*;
use sea_orm::{DbBackend, Statement};
use sea_orm_migration::prelude::*;
use std::fs;

const RESTRICTED_CHARS: [char; 2] = ['#', '%'];

/// Remove `#` and `%` characters from episode file paths.
///
/// - Renames files on disk to remove restricted characters
/// - Updates `file_sub_path` and `image_sub_path` columns in the database
#[derive(DeriveMigrationName)]
pub struct Migration {
    paths: Arc<PathProvider>,
}

impl Migration {
    /// Create a new [`Migration`] using the static [`PATH_PROVIDER`].
    pub fn new() -> Self {
        let paths = PATH_PROVIDER
            .get()
            .expect("PATH_PROVIDER must be set before migrations run")
            .clone();
        Self { paths }
    }

    /// Rename a file to remove restricted characters from its path.
    fn rename_file(podcast_dir: &Path, sub_path: &str) {
        let old = podcast_dir.join(sub_path);
        if !old.exists() {
            return;
        }
        let new_sub_path: String = sub_path
            .chars()
            .filter(|c| !RESTRICTED_CHARS.contains(c))
            .collect();
        let new = podcast_dir.join(&new_sub_path);
        if let Err(e) = fs::rename(&old, &new) {
            warn!("Failed to rename {:?} to {:?}: {}", old, new, e);
        } else {
            info!("Renamed {:?} to {:?}", old, new);
        }
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let podcast_dir = self.paths.get_podcasts_dir();

        let affected = db
            .query_all_raw(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT file_sub_path, image_sub_path FROM episodes
                 WHERE file_sub_path LIKE '%#%'
                    OR file_sub_path LIKE '%!%%' ESCAPE '!'
                    OR image_sub_path LIKE '%#%'
                    OR image_sub_path LIKE '%!%%' ESCAPE '!'",
            ))
            .await?;
        for row in affected {
            if let Ok(path) = row.try_get::<String>("", "file_sub_path") {
                Self::rename_file(&podcast_dir, &path);
            }
            if let Ok(path) = row.try_get::<String>("", "image_sub_path") {
                Self::rename_file(&podcast_dir, &path);
            }
        }

        for char in RESTRICTED_CHARS {
            let like_clause = if char == '%' {
                "LIKE '%!%%' ESCAPE '!'".to_owned()
            } else {
                format!("LIKE '%{char}%'")
            };
            let sql = format!(
                "UPDATE episodes SET file_sub_path = REPLACE(file_sub_path, '{char}', '') WHERE file_sub_path {like_clause}"
            );
            db.execute_unprepared(&sql).await?;
            let sql = format!(
                "UPDATE episodes SET image_sub_path = REPLACE(image_sub_path, '{char}', '') WHERE image_sub_path {like_clause}"
            );
            db.execute_unprepared(&sql).await?;
        }
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        warn!(
            "It's not possible to revert changes to the file_sub_path and image_sub_path. However the database structure is unchanged so this won't cause issues."
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm_migration::SchemaManager;
    use std::fs::write;

    #[test]
    fn rename_file_renames_hash_file() {
        // Arrange
        let dir = TempDirectory::default()
            .create()
            .expect("Should create temp dir");
        let old_path = dir.join("Episode #1.mp3");
        write(&old_path, b"test content").expect("Should write file");

        // Act
        Migration::rename_file(&dir, "Episode #1.mp3");

        // Assert
        assert!(!old_path.exists());
        assert!(dir.join("Episode 1.mp3").exists());
    }

    #[test]
    fn rename_file_skips_missing_file() {
        // Arrange
        let dir = TempDirectory::default()
            .create()
            .expect("Should create temp dir");

        // Act - should not panic
        Migration::rename_file(&dir, "nonexistent #file.mp3");

        // Assert - no file created
        assert!(!dir.join("nonexistent file.mp3").exists());
    }

    #[tokio::test]
    async fn up_renames_dirty_files_and_skips_clean_files() {
        // Arrange
        const DIRTY_FILE: &str = "Episode #1 100%.mp3";
        const DIRTY_IMAGE: &str = "Cover #1 100%.jpg";
        const DIRTY_FILE_AFTER: &str = "Episode 1 100.mp3";
        const DIRTY_IMAGE_AFTER: &str = "Cover 1 100.jpg";
        const CLEAN_FILE: &str = "Episode 2.mp3";
        const CLEAN_IMAGE: &str = "Cover 2.jpg";
        let services = MockServices::new()
            .with_metadata_factory(MockFeedsFactory {
                episodes_per_season: 2,
                edit_episode: Some(|episode| {
                    if episode.episode == Some(1) {
                        episode.file_sub_path =
                            Some(PathWrapper::from_str(DIRTY_FILE).expect("Valid path"));
                        episode.image_sub_path =
                            Some(PathWrapper::from_str(DIRTY_IMAGE).expect("Valid path"));
                    } else {
                        episode.file_sub_path =
                            Some(PathWrapper::from_str(CLEAN_FILE).expect("Valid path"));
                        episode.image_sub_path =
                            Some(PathWrapper::from_str(CLEAN_IMAGE).expect("Valid path"));
                    }
                }),
                ..MockFeedsFactory::default()
            })
            .create()
            .await;
        let paths: Arc<PathProvider> = services.get_async().await.expect("Should get PathProvider");
        let metadata: Arc<MetadataRepository> = services
            .get_async()
            .await
            .expect("Should get MetadataRepository");
        let podcasts_dir = paths.get_podcasts_dir();
        write(podcasts_dir.join(DIRTY_FILE), b"audio").expect("Should write audio file");
        write(podcasts_dir.join(DIRTY_IMAGE), b"image").expect("Should write image file");
        write(podcasts_dir.join(CLEAN_FILE), b"audio").expect("Should write audio file");
        write(podcasts_dir.join(CLEAN_IMAGE), b"image").expect("Should write image file");
        let migration = Migration {
            paths: paths.clone(),
        };
        let manager = SchemaManager::new(&metadata.db);

        // Act
        migration
            .up(&manager)
            .await
            .expect("Migration should succeed");

        // Assert - dirty episode files renamed and database updated
        assert!(!podcasts_dir.join(DIRTY_FILE).exists());
        assert!(!podcasts_dir.join(DIRTY_IMAGE).exists());
        assert!(podcasts_dir.join(DIRTY_FILE_AFTER).exists());
        assert!(podcasts_dir.join(DIRTY_IMAGE_AFTER).exists());
        let dirty_episode = metadata
            .get_episode(MockFeeds::podcast_slug(), 1)
            .await
            .expect("should query episode")
            .expect("episode should exist");
        assert_eq!(
            dirty_episode.file_sub_path,
            Some(PathWrapper::from_str(DIRTY_FILE_AFTER).expect("Valid path"))
        );
        assert_eq!(
            dirty_episode.image_sub_path,
            Some(PathWrapper::from_str(DIRTY_IMAGE_AFTER).expect("Valid path"))
        );
        // Assert - clean episode files and database unchanged
        assert!(podcasts_dir.join(CLEAN_FILE).exists());
        assert!(podcasts_dir.join(CLEAN_IMAGE).exists());
        let clean_episode = metadata
            .get_episode(MockFeeds::podcast_slug(), 2)
            .await
            .expect("should query episode")
            .expect("episode should exist");
        assert_eq!(
            clean_episode.file_sub_path,
            Some(PathWrapper::from_str(CLEAN_FILE).expect("Valid path"))
        );
        assert_eq!(
            clean_episode.image_sub_path,
            Some(PathWrapper::from_str(CLEAN_IMAGE).expect("Valid path"))
        );
    }
}
