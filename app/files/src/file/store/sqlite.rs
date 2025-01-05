use async_trait::async_trait;
use oxidrive_auth::account::AccountId;
use oxidrive_paginate::{Paginate, Slice};
use uuid::Uuid;

use crate::file::{File, FileId};

use super::{AllOwnedByError, ByIdError, ByNameError, FileMetadata, SaveFileError};

pub struct SqliteFileMetadata {
    pool: sqlx::SqlitePool,
}

impl SqliteFileMetadata {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FileMetadata for SqliteFileMetadata {
    async fn all_owned_by(
        &self,
        owner_id: AccountId,
        paginate: Paginate,
    ) -> Result<Slice<File>, AllOwnedByError> {
        let (query, id, limit, is_forward) = match paginate {
            Paginate::Forward { after, first } => (
                r#"
select
  id,
  owner_id,
  name,
  content_type
from files
where owner_id = $1
  and id > $2
order by id
limit $3
"#,
                if after.is_empty() {
                    Uuid::nil().to_string()
                } else {
                    after
                },
                first,
                true,
            ),
            Paginate::Backward { before, last } => (
                r#"
select
  id,
  owner_id,
  name,
  content_type
from files
where owner_id = $1
  and id < $2
order by id desc
limit $3
"#,
                if before.is_empty() {
                    Uuid::max().to_string()
                } else {
                    before
                },
                last,
                false,
            ),
        };

        let files: Vec<SqliteFile> = sqlx::query_as(query)
            .bind(owner_id.to_string())
            .bind(id)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(AllOwnedByError::wrap)?;

        let cursor = files.last().map(|f| f.id.to_string());

        let slice = if is_forward {
            Slice::new(files, cursor, None)
        } else {
            Slice::new(files, None, cursor)
        }
        .map(File::from);
        Ok(slice)
    }

    async fn by_id(&self, owner_id: AccountId, id: FileId) -> Result<Option<File>, ByIdError> {
        let file: Option<SqliteFile> = sqlx::query_as(
            r#"
select
  id,
  owner_id,
  name,
  content_type
from files
where owner_id = $1
  and id = $2
"#,
        )
        .bind(owner_id.to_string())
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(ByIdError::wrap)?;

        Ok(file.map(File::from))
    }

    async fn by_name(
        &self,
        owner_id: AccountId,
        file_name: &str,
    ) -> Result<Option<File>, ByNameError> {
        let file: Option<SqliteFile> = sqlx::query_as(
            r#"
select
  id,
  owner_id,
  name,
  content_type
from files
where owner_id = $1
  and name = $2
"#,
        )
        .bind(owner_id.to_string())
        .bind(file_name)
        .fetch_optional(&self.pool)
        .await
        .map_err(ByNameError::wrap)?;

        Ok(file.map(File::from))
    }

    async fn save(&self, file: File) -> Result<File, SaveFileError> {
        let id = file.id.to_string();
        let owner_id = file.owner_id.to_string();

        sqlx::query(
            r#"
insert into files (
  id,
  owner_id,
  name,
  content_type
) values (
  $1,
  $2,
  $3,
  $4
) on conflict (id)
do update set
  name = excluded.name,
  content_type = excluded.content_type
"#,
        )
        .bind(id)
        .bind(owner_id)
        .bind(&file.name)
        .bind(&file.content_type)
        .execute(&self.pool)
        .await
        .map_err(SaveFileError::wrap)?;

        Ok(file)
    }
}

#[derive(sqlx::FromRow)]
struct SqliteFile {
    id: String,
    owner_id: String,
    name: String,
    content_type: String,
}

impl From<SqliteFile> for File {
    fn from(file: SqliteFile) -> Self {
        Self {
            id: file.id.parse().unwrap(),
            owner_id: file.owner_id.parse().unwrap(),
            name: file.name,
            content_type: file.content_type,
        }
    }
}
