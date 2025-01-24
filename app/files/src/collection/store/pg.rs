use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use oxidrive_auth::account::AccountId;
use oxidrive_database::paginate;
use oxidrive_paginate::{Paginate, Slice};
use sqlx::{Executor, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::{
    collection::{Collection, CollectionId},
    FileId,
};

use super::{AllOwnedByError, ByIdError, CollectionStore, SaveCollectionError};

pub struct PgCollectionStore {
    pool: sqlx::PgPool,
}

impl PgCollectionStore {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CollectionStore for PgCollectionStore {
    async fn all_owned_by(
        &self,
        owner_id: AccountId,
        paginate: Paginate,
    ) -> Result<Slice<Collection>, AllOwnedByError> {
        let mut tx = self.pool.begin().await.map_err(AllOwnedByError::wrap)?;

        let mut qb = QueryBuilder::new(
            "select id, owner_id, name, filter from collections where owner_id = ",
        );
        qb.push_bind(owner_id.as_uuid());

        paginate::postgres::push_query(&mut qb, &paginate, "id");

        let collections: Vec<PgCollection> = qb
            .build_query_as()
            .fetch_all(&mut *tx)
            .await
            .map_err(AllOwnedByError::wrap)?;

        let mut files = self
            .files_for(collections.iter().map(|c| c.id), &mut *tx)
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

        let Some(collection) = sqlx::query_as::<_, PgCollection>(
            "select id, owner_id, name, filter from collections where id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&mut *tx)
        .await
        .map_err(ByIdError::wrap)?
        else {
            return Ok(None);
        };

        let mut collection = Collection::from(collection);

        let mut files = self
            .files_for([id], &mut *tx)
            .await
            .map_err(ByIdError::wrap)?;

        collection.files = files.remove(&id).unwrap_or_default();

        tx.commit().await.map_err(ByIdError::wrap)?;

        Ok(Some(collection))
    }

    async fn save(&self, collection: Collection) -> Result<Collection, SaveCollectionError> {
        let mut tx = self.pool.begin().await.map_err(SaveCollectionError::wrap)?;

        let id = collection.id.as_uuid();

        sqlx::query(
            r#"
insert into collections (
  id,
  owner_id,
  name,
  filter
) values (
  $1,
  $2,
  $3,
  $4
)
on conflict (id)
do update
set
  name = excluded.name,
  filter = excluded.filter
"#,
        )
        .bind(id)
        .bind(collection.owner_id.as_uuid())
        .bind(&collection.name)
        .bind(collection.filter.to_string())
        .execute(&mut *tx)
        .await
        .map_err(SaveCollectionError::wrap)?;

        if !collection.files.is_empty() {
            let mut qb =
                QueryBuilder::new("insert into collections_files (collection_id, file_id) ");

            let files = collection.files.iter().map(|f| f.as_uuid());

            qb.push_values(files.clone(), |mut qb, file_id| {
                qb.push_bind(id).push_bind(file_id);
            });

            qb.push(" on conflict (collection_id, file_id) do nothing");

            qb.build()
                .execute(&mut *tx)
                .await
                .map_err(SaveCollectionError::wrap)?;

            sqlx::query(
                "delete from collections_files where collection_id = $1 and not file_id = any($2)",
            )
            .bind(id)
            .bind(files.collect::<Vec<_>>())
            .execute(&mut *tx)
            .await
            .map_err(SaveCollectionError::wrap)?;
        }

        tx.commit().await.map_err(SaveCollectionError::wrap)?;

        Ok(collection)
    }
}

impl PgCollectionStore {
    async fn files_for<'e, Id, I, E>(
        &self,
        ids: I,
        executor: E,
    ) -> sqlx::Result<HashMap<CollectionId, HashSet<FileId>>>
    where
        Id: Into<Uuid>,
        I: IntoIterator<Item = Id>,
        E: Executor<'e, Database = Postgres>,
    {
        let ids: Vec<(Uuid, Uuid)> = sqlx::query_as(
            "select collection_id, file_id from collections_files where collection_id = ANY($1)",
        )
        .bind(ids.into_iter().map(Into::into).collect::<Vec<_>>())
        .fetch_all(executor)
        .await?;

        let ids = ids
            .into_iter()
            .map(|(cid, fid)| (CollectionId::from(cid), FileId::from(fid)))
            .fold(HashMap::new(), |mut map, (cid, fid)| {
                let files: &mut HashSet<FileId> = map.entry(cid).or_default();
                files.insert(fid);
                map
            });
        Ok(ids)
    }
}

#[derive(Debug, sqlx::FromRow)]
struct PgCollection {
    id: Uuid,
    owner_id: Uuid,
    name: String,
    filter: String,
}

impl From<PgCollection> for Collection {
    fn from(collection: PgCollection) -> Self {
        Self {
            id: collection.id.into(),
            name: collection.name,
            owner_id: collection.owner_id.into(),
            filter: collection.filter.parse().unwrap(),
            files: Default::default(),
        }
    }
}
