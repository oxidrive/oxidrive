use std::sync::Arc;

use oxidrive_auth::account::AccountId;
use oxidrive_paginate::Paginate;
use oxidrive_workers::{Dispatch, DispatchError, Job, Process};
use serde::{Deserialize, Serialize};

use crate::collection::{AllOwnedByError, CollectionStore};

use super::refresh_collection::RefreshCollection;

#[derive(Clone)]
pub struct RefreshCollectionsWorker {
    refresh: Dispatch<RefreshCollection>,
    collections: Arc<dyn CollectionStore>,
}

impl RefreshCollectionsWorker {
    pub fn new(
        refresh: Dispatch<RefreshCollection>,
        collections: Arc<dyn CollectionStore>,
    ) -> Self {
        Self {
            refresh,
            collections,
        }
    }
}

impl Process for RefreshCollectionsWorker {
    type Job = RefreshCollections;

    type Error = RefreshCollectionsError;

    async fn process(&self, job: Self::Job) -> Result<(), Self::Error> {
        loop {
            let collections = self
                .collections
                .all_owned_by(job.owner_id, Paginate::default())
                .await?;

            if collections.is_empty() || collections.next.is_none() {
                return Ok(());
            }

            for collection in collections {
                self.refresh
                    .dispatch(RefreshCollection {
                        collection_id: collection.id,
                    })
                    .await?;
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshCollections {
    pub owner_id: AccountId,
}

impl Job for RefreshCollections {}

#[derive(Debug, thiserror::Error)]
pub enum RefreshCollectionsError {
    #[error("failed to load collections to refresh: {0}")]
    LoadFailed(#[from] AllOwnedByError),

    #[error(transparent)]
    DispatchFailed(#[from] DispatchError),
}
