use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use oxidrive_accounts::account::AccountId;
use oxidrive_database::paginate;
use oxidrive_paginate::{Paginate, Slice};
use sqlx::{Executor, QueryBuilder, Sqlite};

use crate::{
    FileId,
    collection::{Collection, CollectionId},
};

use super::{AllOwnedByError, ByIdError, CollectionStore, SaveCollectionError};

pub struct SqliteCollectionStore {
    pool: sqlx::SqlitePool,
}

impl SqliteCollectionStore {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CollectionStore for SqliteCollectionStore {
    async fn all_owned_by(
        &self,
        owner_id: AccountId,
        paginate: Paginate,
    ) -> Result<Slice<Collection>, AllOwnedByError> {
        let mut tx = self.pool.begin().await.map_err(AllOwnedByError::wrap)?;

        let mut qb = QueryBuilder::new(
            "select id, owner_id, name, filter from collections where owner_id = ",
        );
        qb.push_bind(owner_id.to_string());

        paginate::sqlite::push_query(&mut qb, &paginate, "id");

        let collections: Vec<SqliteCollection> = qb
            .build_query_as()
            .fetch_all(&mut *tx)
            .await
            .map_err(AllOwnedByError::wrap)?;

        let mut files = self
            .files_for(collections.iter().map(|c| &c.id), &mut *tx)
            .await
            .map_err(AllOwnedByError::wrap)?;

        tx.commit().await.map_err(AllOwnedByError::wrap)?;

        let mut slice =
            paginate::to_slice(collections, |c| c.id.to_string(), &paginate).map(Collection::from);

        for collection in slice.items.iter_mut() {
            collection.files = files.remove(&collection.id).unwrap_or_default();
        }

        Ok(slice)
    }

    async fn by_id(&self, id: CollectionId) -> Result<Option<Collection>, ByIdError> {
        let mut tx = self.pool.begin().await.map_err(ByIdError::wrap)?;

        let Some(collection) = sqlx::query_as::<_, SqliteCollection>(
            "select id, owner_id, name, filter from collections where id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&mut *tx)
        .await
        .map_err(ByIdError::wrap)?
        else {
            return Ok(None);
        };

        let mut collection = Collection::from(collection);

        let mut files = self
            .files_for(&[id], &mut *tx)
            .await
            .map_err(ByIdError::wrap)?;

        collection.files = files.remove(&id).unwrap_or_default();

        tx.commit().await.map_err(ByIdError::wrap)?;

        Ok(Some(collection))
    }

    async fn save(&self, collection: Collection) -> Result<Collection, SaveCollectionError> {
        let mut tx = self.pool.begin().await.map_err(SaveCollectionError::wrap)?;

        let id = collection.id.to_string();

        sqlx::query(
            r#"
insert into collections (
  id,
  owner_id,
  name,
  filter
) values (
  ?,
  ?,
  ?,
  ?
)
on conflict (id)
do update
set
  name = excluded.name,
  filter = excluded.filter
"#,
        )
        .bind(&id)
        .bind(collection.owner_id.to_string())
        .bind(&collection.name)
        .bind(collection.filter.to_string())
        .execute(&mut *tx)
        .await
        .map_err(SaveCollectionError::wrap)?;

        if !collection.files.is_empty() {
            let mut qb =
                QueryBuilder::new("insert into collections_files (collection_id, file_id) ");

            let files = collection.files.iter().map(|f| f.to_string());

            qb.push_values(files.clone(), |mut qb, file_id| {
                qb.push_bind(&id).push_bind(file_id);
            });

            qb.push(" on conflict (collection_id, file_id) do nothing");

            qb.build()
                .execute(&mut *tx)
                .await
                .map_err(SaveCollectionError::wrap)?;

            let mut qb = QueryBuilder::new("delete from collections_files where collection_id = ");

            qb.push_bind(&id).push(" and file_id not in (");

            let mut s = qb.separated(", ");

            for file in files {
                s.push_bind(file);
            }

            qb.push(")");

            qb.build()
                .execute(&mut *tx)
                .await
                .map_err(SaveCollectionError::wrap)?;
        }

        tx.commit().await.map_err(SaveCollectionError::wrap)?;

        Ok(collection)
    }
}

impl SqliteCollectionStore {
    async fn files_for<'e, Id, I, E>(
        &self,
        ids: I,
        executor: E,
    ) -> sqlx::Result<HashMap<CollectionId, HashSet<FileId>>>
    where
        Id: ToString,
        I: IntoIterator<Item = Id>,
        E: Executor<'e, Database = Sqlite>,
    {
        let mut qb = QueryBuilder::new(
            "select collection_id, file_id from collections_files where collection_id in (",
        );

        let mut s = qb.separated(", ");

        for id in ids.into_iter().map(|id| id.to_string()) {
            s.push_bind(id);
        }

        qb.push(")");

        let ids: Vec<(String, String)> = qb.build_query_as().fetch_all(executor).await?;

        let ids = ids
            .into_iter()
            .map(|(cid, fid)| (cid.parse().unwrap(), fid.parse().unwrap()))
            .fold(HashMap::new(), |mut map, (cid, fid)| {
                let files: &mut HashSet<FileId> = map.entry(cid).or_default();
                files.insert(fid);
                map
            });
        Ok(ids)
    }
}

#[derive(Debug, sqlx::FromRow)]
struct SqliteCollection {
    id: String,
    owner_id: String,
    name: String,
    filter: String,
}

impl From<SqliteCollection> for Collection {
    fn from(collection: SqliteCollection) -> Self {
        Self {
            id: collection.id.parse().unwrap(),
            name: collection.name,
            owner_id: collection.owner_id.parse().unwrap(),
            filter: collection.filter.parse().unwrap(),
            files: Default::default(),
        }
    }
}
