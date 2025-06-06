use std::collections::BTreeMap;

use async_trait::async_trait;
use oxidrive_accounts::account::AccountId;
use oxidrive_database::paginate;
use oxidrive_paginate::{Paginate, Slice};
use oxidrive_search::Filter;
use sqlx::{QueryBuilder, types::Json};

use crate::{
    Tag,
    file::{File, FileId, Tags},
};

use super::{
    AllOwnedByInError, ByIdError, ByNameError, DeleteFileError, FileMetadata, SaveFileError,
    SearchError,
};

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
    async fn all_owned_by_in(
        &self,
        owner_id: AccountId,
        ids: &[FileId],
        paginate: Paginate,
    ) -> Result<Slice<File>, AllOwnedByInError> {
        let mut qb = QueryBuilder::new(
            r#"
select distinct
  id,
  owner_id,
  name,
  content_type,
  size,
  tags,
  hash
from files
where owner_id ="#,
        );

        qb.push_bind(owner_id.to_string());

        qb.push(" and id in (");

        let mut values = qb.separated(", ");
        for id in ids {
            values.push_bind(id.to_string());
        }

        values.push_unseparated(")");

        paginate::sqlite::push_query(&mut qb, &paginate, "lower(name)");

        let files: Vec<SqliteFile> = qb
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(AllOwnedByInError::wrap)?;

        let slice = paginate::to_slice(files, |f| f.id.to_string(), &paginate).map(File::from);
        Ok(slice)
    }

    async fn by_id(&self, id: FileId) -> Result<Option<File>, ByIdError> {
        let file: Option<SqliteFile> = sqlx::query_as(
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
        .bind(id.to_string())
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
        let file: Option<SqliteFile> = sqlx::query_as(
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
        .bind(id)
        .bind(owner_id)
        .bind(&file.name)
        .bind(&file.content_type)
        .bind(file.size as i64)
        .bind(to_sqlite_tags(file.tags.clone()))
        .bind(file.hash.as_ref().map(|hash| hash.as_bytes().as_slice()))
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
select distinct
  id,
  owner_id,
  name,
  content_type,
  size,
  tags,
  hash
from files
where owner_id ="#,
        );

        qb.push_bind(owner_id.to_string());

        push_search_query(&mut qb, filter);

        paginate::sqlite::push_query(&mut qb, &paginate, "lower(name)");

        let files: Vec<SqliteFile> = qb
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(SearchError::wrap)?;

        let slice = paginate::to_slice(files, |f| f.id.to_string(), &paginate).map(File::from);
        Ok(slice)
    }

    async fn delete(&self, id: FileId) -> Result<(), DeleteFileError> {
        sqlx::query("delete from files where id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(DeleteFileError::wrap)?;
        Ok(())
    }
}

fn push_search_query(qb: &mut QueryBuilder<'_, sqlx::Sqlite>, filter: Filter) {
    qb.push(" and (");
    traverse_query(qb, filter);
    qb.push(")");
}

fn traverse_query(qb: &mut QueryBuilder<'_, sqlx::Sqlite>, filter: Filter) {
    match filter {
        Filter::All => {
            qb.push("1=1");
        }
        Filter::Tag { key, values } => {
            if values.is_empty() {
                qb.push(format!("tags->>'{key}' is not null"));
                return;
            }

            let value = values.to_string().replace('"', "");

            if values.has_matches() {
                let value = value.replace('*', "%");
                qb.push(format!("tags->>'{key}' LIKE ")).push_bind(value);
            } else {
                qb.push(format!("tags->>'{key}' = ")).push_bind(value);
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
struct SqliteFile {
    id: String,
    owner_id: String,
    name: String,
    content_type: String,
    size: i64,
    tags: SqliteTags,
    hash: Option<Vec<u8>>,
}

impl From<SqliteFile> for File {
    fn from(file: SqliteFile) -> Self {
        Self {
            id: file.id.parse().unwrap(),
            owner_id: file.owner_id.parse().unwrap(),
            name: file.name,
            content_type: file.content_type,
            size: file.size.try_into().unwrap(),
            tags: file
                .tags
                .0
                .into_iter()
                .map(|(key, value)| match value {
                    serde_json::Value::String(value) => Tag::full(key, value),
                    serde_json::Value::Object(_) => Tag::key(key),
                    _ => unreachable!(),
                })
                .map(Tag::into)
                .collect(),
            hash: file
                .hash
                .map(|bytes| blake3::Hash::from_bytes(bytes.try_into().unwrap())),
        }
    }
}

type SqliteTags = Json<BTreeMap<String, serde_json::Value>>;

fn to_sqlite_tags(tags: Tags) -> SqliteTags {
    let tags = tags
        .into_values()
        .map(|tag| match tag.value {
            Some(value) => (tag.key, serde_json::Value::String(value)),
            None => (tag.key, serde_json::Value::Object(Default::default())),
        })
        .collect();

    Json(tags)
}
