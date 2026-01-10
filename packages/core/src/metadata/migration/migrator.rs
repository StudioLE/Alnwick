use crate::metadata::migration::*;
use sea_orm_migration::async_trait::async_trait;
use sea_orm_migration::prelude::*;

/// `SeaORM` database migrator.
pub(crate) struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251119_001_create_podcasts_table::Migration),
            Box::new(m20251119_002_create_episodes_table::Migration),
            Box::new(m20260107_001_add_feed_url_to_podcasts::Migration),
            Box::new(m20260110_001_sanitize_episode_paths::Migration::new()),
        ]
    }
}
