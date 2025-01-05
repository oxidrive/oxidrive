use async_trait::async_trait;
use oxidrive_files::{file::FileId, File};
use sqlx::QueryBuilder;

use crate::Tag;

use super::{ForFileError, IndexTagError, TagIndex};

pub struct SqliteTagIndex {
    pool: sqlx::SqlitePool,
}

impl SqliteTagIndex {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TagIndex for SqliteTagIndex {
    async fn index(&self, file: &File, tags: Vec<Tag>) -> Result<(), IndexTagError> {
        let mut qb = QueryBuilder::new("insert or ignore into tags (key, value, file_id)");

        qb.push_values(tags, |mut qb, tag| {
            qb.push_bind(tag.key)
                .push_bind(tag.value)
                .push_bind(file.id.to_string());
        });

        qb.build()
            .execute(&self.pool)
            .await
            .map_err(IndexTagError::wrap)?;
        Ok(())
    }

    async fn for_file(&self, file_id: FileId) -> Result<Vec<Tag>, ForFileError> {
        let tags: Vec<(String, Option<String>)> =
            sqlx::query_as("select key, value from tags where file_id = $1")
                .bind(file_id.to_string())
                .fetch_all(&self.pool)
                .await
                .map_err(ForFileError::wrap)?;

        Ok(tags
            .into_iter()
            .map(|(key, value)| Tag { key, value })
            .collect())
    }
}
