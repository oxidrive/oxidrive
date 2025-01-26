use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use oxidrive_accounts::account::AccountId;
use oxidrive_domain::make_error_wrapper;
use oxidrive_paginate::{Paginate, Slice};
use tokio::sync::RwLock;
use uuid::Uuid;

use super::{Collection, CollectionId};

pub use pg::*;
pub use sqlite::*;

mod pg;
mod sqlite;

make_error_wrapper!(AllOwnedByError);
make_error_wrapper!(ByIdError);
make_error_wrapper!(SaveCollectionError);

#[async_trait]
pub trait CollectionStore: Send + Sync + 'static {
    async fn all_owned_by(
        &self,
        owner_id: AccountId,
        paginate: Paginate,
    ) -> Result<Slice<Collection>, AllOwnedByError>;

    async fn by_id(&self, id: CollectionId) -> Result<Option<Collection>, ByIdError>;

    async fn save(&self, collection: Collection) -> Result<Collection, SaveCollectionError>;
}

#[derive(Clone, Default)]
pub struct InMemoryCollectionStore {
    inner: Arc<RwLock<HashMap<CollectionId, Collection>>>,
}

impl<const N: usize> From<[Collection; N]> for InMemoryCollectionStore {
    fn from(files: [Collection; N]) -> Self {
        let files = HashMap::from_iter(files.into_iter().map(|f| (f.id, f)));
        Self {
            inner: Arc::new(RwLock::new(files)),
        }
    }
}

#[async_trait]
impl CollectionStore for InMemoryCollectionStore {
    async fn all_owned_by(
        &self,
        owner_id: AccountId,
        paginate: Paginate,
    ) -> Result<Slice<Collection>, AllOwnedByError> {
        let inner = self.inner.read().await;

        let (id, limit, is_forward) = match paginate {
            Paginate::Forward { after, first } => (
                if after.is_empty() {
                    Uuid::nil().to_string()
                } else {
                    after
                },
                first,
                true,
            ),
            Paginate::Backward { before, last } => (
                if before.is_empty() {
                    Uuid::max().to_string()
                } else {
                    before
                },
                last,
                false,
            ),
        };

        let mut collections: Vec<Collection> = inner
            .values()
            .filter(|c| c.owner_id == owner_id)
            .filter(|c| {
                if is_forward {
                    c.id.to_string() > id
                } else {
                    c.id.to_string() < id
                }
            })
            .take(limit)
            .cloned()
            .collect();

        collections.sort_by_key(|c| c.id);

        if is_forward {
            let next = collections.last().map(|f| f.id.to_string());
            Ok(Slice::new(collections, next, None))
        } else {
            let previous = collections.first().map(|f| f.id.to_string());
            Ok(Slice::new(collections, None, previous))
        }
    }

    async fn by_id(&self, id: CollectionId) -> Result<Option<Collection>, ByIdError> {
        let inner = self.inner.read().await;
        Ok(inner.get(&id).cloned())
    }

    async fn save(&self, collection: Collection) -> Result<Collection, SaveCollectionError> {
        let mut inner = self.inner.write().await;
        inner.insert(collection.id, collection.clone());
        Ok(collection)
    }
}

#[cfg(test)]
mod tests;
