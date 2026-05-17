use sea_orm::{DatabaseBackend, Statement};
use sqlformat::{FormatOptions, QueryParams, format};

pub const DB_BACKEND: DatabaseBackend = DatabaseBackend::Sqlite;

#[must_use]
pub(crate) fn format_sql(statement: &Statement) -> String {
    format(
        &statement.to_string(),
        &QueryParams::None,
        &FormatOptions::default(),
    )
}
