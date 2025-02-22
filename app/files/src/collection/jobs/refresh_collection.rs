use std::{collections::HashSet, sync::Arc};

use oxidrive_paginate::Paginate;
use oxidrive_workers::{Job, Process};
use serde::{Deserialize, Serialize};

use crate::{
    FileId,
    collection::{ByIdError, CollectionId, CollectionStore, SaveCollectionError},
    file::{FileMetadata, SearchError},
};

#[derive(Clone)]
pub struct RefreshCollectionWorker {
    files: Arc<dyn FileMetadata>,
    collections: Arc<dyn CollectionStore>,
}

impl RefreshCollectionWorker {
    pub fn new(files: Arc<dyn FileMetadata>, collections: Arc<dyn CollectionStore>) -> Self {
        Self { files, collections }
    }
}

impl Process for RefreshCollectionWorker {
    type Job = RefreshCollection;

    type Error = RefreshCollectionError;

    async fn process(&self, job: Self::Job) -> Result<(), Self::Error> {
        let Some(mut collection) = self.collections.by_id(job.collection_id).await? else {
            tracing::warn!(collection_id = %job.collection_id, "could not refresh collection as it doesn't seem to exist");
            return Ok(());
        };

        let existing = collection.files.drain().collect::<HashSet<FileId>>();

        let mut paginate = Paginate::default();

        loop {
            let files = self
                .files
                .search(collection.owner_id, collection.filter.clone(), paginate)
                .await?;

            if files.is_empty() || files.next.is_none() {
                break;
            }

            let next = files.next.clone().unwrap();

            collection.add(files.into_iter().map(|f| f.id));

            paginate = Paginate::after(next);
        }

        if collection.files == existing {
            tracing::trace!(collection_id = %collection.id, "collection has not changed, skipping update");
            return Ok(());
        }

        self.collections.save(collection).await?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshCollection {
    pub collection_id: CollectionId,
}

impl Job for RefreshCollection {}

#[derive(Debug, thiserror::Error)]
pub enum RefreshCollectionError {
    #[error("fails to load collection: {0}")]
    LoadFailed(#[from] ByIdError),

    #[error("fails to search files: {0}")]
    SearchFailed(#[from] SearchError),

    #[error("fails to save collection: {0}")]
    SaveFailed(#[from] SaveCollectionError),
}

#[cfg(test)]
mod tests {
    use assert2::check;

    use oxidrive_search::Filter;
    use rstest::rstest;

    use crate::{
        File,
        collection::{Collection, InMemoryCollectionStore},
        file::{InMemoryFileMetadata, fixtures::file},
    };

    use super::*;

    #[rstest]
    #[tokio::test]
    async fn it_refreshes_a_collection(#[from(file)] file1: File, #[from(file)] file2: File) {
        let collection = Collection::new(file1.owner_id, "test", Filter::All);
        let collection_id = collection.id;

        let file1_id = file1.id;

        let files = Arc::new(InMemoryFileMetadata::from([file1, file2]));
        let collections = Arc::new(InMemoryCollectionStore::from([collection]));

        let worker = RefreshCollectionWorker::new(files, collections.clone());

        worker
            .process(RefreshCollection { collection_id })
            .await
            .unwrap();

        let collection = collections.by_id(collection_id).await.unwrap().unwrap();
        check!(collection.files.contains(&file1_id));
    }
}
