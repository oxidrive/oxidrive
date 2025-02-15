use async_trait::async_trait;
use oxidrive_accounts::account::AccountId;
use oxidrive_database::paginate;
use oxidrive_paginate::{Paginate, Slice};
use oxidrive_search::Filter;
use sqlx::{postgres::types::PgHstore, QueryBuilder};
use uuid::Uuid;

use crate::{
    file::{File, FileId},
    Tag,
};

use super::{
    AllOwnedByInError, ByIdError, ByNameError, DeleteFileError, FileMetadata, SaveFileError,
    SearchError,
};

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
    async fn all_owned_by_in(
        &self,
        owner_id: AccountId,
        ids: &[FileId],
        paginate: Paginate,
    ) -> Result<Slice<File>, AllOwnedByInError> {
        let mut qb = QueryBuilder::new(
            r#"
select
  id,
  owner_id,
  name,
  content_type,
  size,
  tags,
  hash
from files
where owner_id =
"#,
        );

        qb.push_bind(owner_id.as_uuid());

        qb.push(" and id = any(")
            .push_bind(ids.iter().map(FileId::as_uuid).collect::<Vec<Uuid>>())
            .push(")");

        paginate::postgres::push_query(&mut qb, &paginate, "lower(name)");

        let files: Vec<PgFile> = qb
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(AllOwnedByInError::wrap)?;

        let slice = paginate::to_slice(files, |f| f.id.to_string(), &paginate).map(File::from);
        Ok(slice)
    }

    async fn by_id(&self, id: FileId) -> Result<Option<File>, ByIdError> {
        let file: Option<PgFile> = sqlx::query_as(
            r#"
select
  id,
  owner_id,
  name,
  content_type,
  size,
  tags,
  hash
from files
where id = $1
"#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(ByIdError::wrap)?;

        Ok(file.map(File::from))
    }

    async fn by_owner_and_name(
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
  content_type,
  size,
  tags,
  hash
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
  content_type,
  size,
  tags,
  hash
) values (
  $1,
  $2,
  $3,
  $4,
  $5,
  $6,
  $7
) on conflict (id)
do update set
  name = excluded.name,
  content_type = excluded.content_type,
  size = excluded.size,
  tags = excluded.tags,
  hash = excluded.hash
"#,
        )
        .bind(file.id.as_uuid())
        .bind(file.owner_id.as_uuid())
        .bind(&file.name)
        .bind(&file.content_type)
        .bind(file.size as i64)
        .bind(PgHstore(
            file.tags
                .clone()
                .into_iter()
                .map(|(key, tag)| (key, tag.value))
                .collect(),
        ))
        .bind(file.hash.as_ref().map(blake3::Hash::as_bytes))
        .execute(&self.pool)
        .await
        .map_err(SaveFileError::wrap)?;

        Ok(file)
    }

    async fn search(
        &self,
        owner_id: AccountId,
        filter: Filter,
        paginate: Paginate,
    ) -> Result<Slice<File>, SearchError> {
        let mut qb = QueryBuilder::new(
            r#"
select
  id,
  owner_id,
  name,
  content_type,
  size,
  tags,
  hash
from files
where owner_id =
"#,
        );

        qb.push_bind(owner_id.as_uuid());

        push_search_query(&mut qb, filter);

        paginate::postgres::push_query(&mut qb, &paginate, "lower(name)");

        let files: Vec<PgFile> = qb
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(SearchError::wrap)?;

        let slice = paginate::to_slice(files, |f| f.id.to_string(), &paginate).map(File::from);
        Ok(slice)
    }

    async fn delete(&self, id: FileId) -> Result<(), DeleteFileError> {
        sqlx::query("delete from files where id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(DeleteFileError::wrap)?;
        Ok(())
    }
}

fn push_search_query(qb: &mut QueryBuilder<'_, sqlx::Postgres>, filter: Filter) {
    qb.push(" and (");
    traverse_query(qb, filter);
    qb.push(")");
}

fn traverse_query(qb: &mut QueryBuilder<'_, sqlx::Postgres>, filter: Filter) {
    match filter {
        Filter::All => {
            qb.push("1=1");
        }
        Filter::Tag { key, values } => {
            if values.is_empty() {
                qb.push("tags ? ").push_bind(key);
                return;
            }

            let value = values.to_string().replace('"', "");

            if values.has_matches() {
                let value = value.replace('*', "%");
                qb.push(format!("tags->'{key}' LIKE ")).push_bind(value);
            } else {
                qb.push(format!("tags->'{key}' = ")).push_bind(value);
            }
        }
        Filter::Op { lhs, op, rhs } => {
            qb.push("(");
            traverse_query(qb, *lhs);
            qb.push(") ");

            match op {
                oxidrive_search::Op::And => qb.push(" and "),
                oxidrive_search::Op::Or => qb.push(" or "),
            };

            qb.push("(");
            traverse_query(qb, *rhs);
            qb.push(") ");
        }
        Filter::Mod { modifier, inner } => {
            match modifier {
                oxidrive_search::Mod::Not => qb.push(" not "),
            };

            traverse_query(qb, *inner);
        }
    }
}

#[derive(sqlx::FromRow)]
struct PgFile {
    id: Uuid,
    owner_id: Uuid,
    name: String,
    content_type: String,
    size: i64,
    tags: PgHstore,
    hash: Option<Vec<u8>>,
}

impl From<PgFile> for File {
    fn from(file: PgFile) -> Self {
        Self {
            id: file.id.into(),
            owner_id: file.owner_id.into(),
            name: file.name,
            content_type: file.content_type,
            size: file.size.try_into().unwrap(),
            tags: file
                .tags
                .0
                .into_iter()
                .map(Tag::from)
                .map(Tag::into)
                .collect(),
            hash: file
                .hash
                .map(|bytes| blake3::Hash::from_bytes(bytes.try_into().unwrap())),
        }
    }
}
