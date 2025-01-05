use async_trait::async_trait;
use oxidrive_files::{file::FileId, File};
use sqlx::QueryBuilder;

use crate::Tag;

use super::{ForFileError, IndexTagError, TagIndex};

pub struct PgTagIndex {
    pool: sqlx::PgPool,
}

impl PgTagIndex {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TagIndex for PgTagIndex {
    async fn index(&self, file: &File, tags: Vec<Tag>) -> Result<(), IndexTagError> {
        let mut qb = QueryBuilder::new("insert into tags (key, value, file_id)");

        qb.push_values(tags, |mut qb, tag| {
            qb.push_bind(tag.key)
                .push_bind(tag.value)
                .push_bind(file.id.as_uuid());
        });

        qb.push("on conflict (key, value, file_id) do nothing")
            .build()
            .execute(&self.pool)
            .await
            .map_err(IndexTagError::wrap)?;
        Ok(())
    }

    async fn for_file(&self, file_id: FileId) -> Result<Vec<Tag>, ForFileError> {
        let tags: Vec<(String, Option<String>)> =
            sqlx::query_as("select key, value from tags where file_id = $1")
                .bind(file_id.as_uuid())
                .fetch_all(&self.pool)
                .await
                .map_err(ForFileError::wrap)?;

        Ok(tags
            .into_iter()
            .map(|(key, value)| Tag { key, value })
            .collect())
    }
}
