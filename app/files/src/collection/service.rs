use std::sync::Arc;

use oxidrive_auth::account::AccountId;
use oxidrive_paginate::{Paginate, Slice};
use oxidrive_search::QueryParseError;
use oxidrive_workers::Dispatch;

use super::{
    jobs::RefreshCollection, AllOwnedByError, ByIdError, Collection, CollectionId, CollectionStore,
    SaveCollectionError,
};

#[derive(Clone)]
pub struct Collections {
    collections: Arc<dyn CollectionStore>,
    refresh: Dispatch<RefreshCollection>,
}

impl Collections {
    pub fn new(
        collections: Arc<dyn CollectionStore>,
        refresh: Dispatch<RefreshCollection>,
    ) -> Self {
        Self {
            collections,
            refresh,
        }
    }

    pub async fn all_owned_by(
        &self,
        owner_id: AccountId,
        paginate: Paginate,
    ) -> Result<Slice<Collection>, AllOwnedByError> {
        self.collections.all_owned_by(owner_id, paginate).await
    }

    pub async fn create(
        &self,
        owner_id: AccountId,
        data: CreateCollection,
    ) -> Result<Collection, CreateCollectionError> {
        let CreateCollection { name, filter } = data;
        let filter = oxidrive_search::parse_query(filter)?;
        let collection = Collection::new(owner_id, name, filter);
        let collection = self.collections.save(collection).await?;

        let _ = self
            .refresh
            .dispatch(RefreshCollection {
                collection_id: collection.id,
            })
            .await;

        Ok(collection)
    }

    pub async fn by_id(&self, id: CollectionId) -> Result<Option<Collection>, ByIdError> {
        self.collections.by_id(id).await
    }

    pub async fn update(
        &self,
        id: CollectionId,
        data: UpdateCollection,
    ) -> Result<Collection, UpdateCollectionError> {
        let Some(mut collection) = self.collections.by_id(id).await? else {
            return Err(UpdateCollectionError::NotFound(id));
        };

        if let Some(name) = data.name {
            collection.name = name;
        }

        if let Some(filter) = data.filter {
            collection.filter = oxidrive_search::parse_query(filter)?;
        }

        let collection = self.collections.save(collection).await?;

        let _ = self
            .refresh
            .dispatch(RefreshCollection {
                collection_id: collection.id,
            })
            .await;

        Ok(collection)
    }
}

pub struct CreateCollection {
    pub name: String,
    pub filter: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateCollectionError {
    #[error(transparent)]
    FilterParse(#[from] QueryParseError),
    #[error(transparent)]
    SaveFailed(#[from] SaveCollectionError),
}

pub struct UpdateCollection {
    pub name: Option<String>,
    pub filter: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateCollectionError {
    #[error("no collection found by id {0})")]
    NotFound(CollectionId),
    #[error(transparent)]
    LoadFailed(#[from] ByIdError),
    #[error(transparent)]
    FilterParse(#[from] QueryParseError),
    #[error(transparent)]
    SaveFailed(#[from] SaveCollectionError),
}
