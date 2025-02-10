use std::sync::Arc;

use oxidrive_accounts::account::AccountId;
use oxidrive_paginate::{Paginate, Slice};
use oxidrive_pubsub::Publisher;
use oxidrive_search::QueryParseError;

use super::{
    AllOwnedByError, ByIdError, Collection, CollectionId, CollectionStore, SaveCollectionError,
};

pub use event::*;

mod event;

#[derive(Clone)]
pub struct Collections {
    collections: Arc<dyn CollectionStore>,
    publisher: Publisher<CollectionEvent>,
}

impl Collections {
    pub fn new(
        collections: Arc<dyn CollectionStore>,
        publisher: Publisher<CollectionEvent>,
    ) -> Self {
        Self {
            collections,
            publisher,
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

        self.publisher
            .publish(CollectionEvent::Changed(collection.clone()));

        Ok(collection)
    }

    pub async fn by_id(&self, id: CollectionId) -> Result<Option<Collection>, ByIdError> {
        self.collections.by_id(id).await
    }

    pub async fn update(
        &self,
        mut collection: Collection,
        data: UpdateCollection,
    ) -> Result<Collection, UpdateCollectionError> {
        if let Some(name) = data.name {
            collection.name = name;
        }

        if let Some(filter) = data.filter {
            collection.filter = oxidrive_search::parse_query(filter)?;
        }

        let collection = self.collections.save(collection).await?;

        self.publisher
            .publish(CollectionEvent::Changed(collection.clone()));

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
    #[error(transparent)]
    FilterParse(#[from] QueryParseError),
    #[error(transparent)]
    SaveFailed(#[from] SaveCollectionError),
}
