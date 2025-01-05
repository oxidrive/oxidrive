use async_trait::async_trait;
use oxidrive_auth::account::AccountId;
use oxidrive_paginate::{Paginate, Slice};
use uuid::Uuid;

use crate::file::{File, FileId};

use super::{AllOwnedByError, ByIdError, ByNameError, FileMetadata, SaveFileError};

pub struct PgFileMetadata {
    pool: sqlx::PgPool,
}

impl PgFileMetadata {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FileMetadata for PgFileMetadata {
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
  and id::text > $2
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
  and id::text < $2
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

        let files: Vec<PgFile> = sqlx::query_as(query)
            .bind(owner_id.as_uuid())
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
        let file: Option<PgFile> = sqlx::query_as(
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
        .bind(owner_id.as_uuid())
        .bind(id.as_uuid())
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
        let file: Option<PgFile> = sqlx::query_as(
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
        .bind(owner_id.as_uuid())
        .bind(file_name)
        .fetch_optional(&self.pool)
        .await
        .map_err(ByNameError::wrap)?;

        Ok(file.map(File::from))
    }

    async fn save(&self, file: File) -> Result<File, SaveFileError> {
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
        .bind(file.id.as_uuid())
        .bind(file.owner_id.as_uuid())
        .bind(&file.name)
        .bind(&file.content_type)
        .execute(&self.pool)
        .await
        .map_err(SaveFileError::wrap)?;

        Ok(file)
    }
}

#[derive(sqlx::FromRow)]
struct PgFile {
    id: Uuid,
    owner_id: Uuid,
    name: String,
    content_type: String,
}

impl From<PgFile> for File {
    fn from(file: PgFile) -> Self {
        Self {
            id: file.id.into(),
            owner_id: file.owner_id.into(),
            name: file.name,
            content_type: file.content_type,
        }
    }
}
