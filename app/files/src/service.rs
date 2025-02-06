use std::sync::Arc;

use crate::{
    collection::jobs::RefreshCollections,
    content_type,
    file::{
        self, ByNameError, DownloadFileError, FileMetadata, FileStorage, SaveFileError,
        UploadFileError,
    },
    File, Tag,
};
use bytes::Bytes;
use futures::Stream;
use oxidrive_accounts::account::AccountId;
use oxidrive_paginate::{Paginate, Slice};
use oxidrive_search::QueryParseError;
use oxidrive_workers::Dispatch;

#[derive(Clone)]
pub struct Files {
    metadata: Arc<dyn FileMetadata>,
    storage: FileStorage,

    refresh_collection: Dispatch<RefreshCollections>,
}

impl Files {
    pub fn new(
        files: Arc<dyn FileMetadata>,
        storage: FileStorage,
        refresh_collection: Dispatch<RefreshCollections>,
    ) -> Self {
        Self {
            metadata: files,
            storage,
            refresh_collection,
        }
    }

    pub fn metadata(&self) -> &dyn FileMetadata {
        self.metadata.as_ref()
    }

    pub async fn download(
        &self,
        file: &File,
    ) -> Result<
        Option<impl Stream<Item = Result<Bytes, impl std::error::Error>> + 'static>,
        DownloadError,
    > {
        Ok(self.storage.download(file).await?)
    }

    pub async fn upload<C, E>(&self, meta: UploadMetadata, content: C) -> Result<File, UploadError>
    where
        C: Stream<Item = Result<Bytes, E>> + Unpin + Send,
        E: std::error::Error + Send + Sync + 'static,
    {
        let (content, content_type) =
            content_type::detect_from_stream(&meta.file_name, content).await;

        let mut file = match self
            .metadata
            .by_owner_and_name(meta.owner_id, &meta.file_name)
            .await?
        {
            Some(mut file) => {
                file.content_type = content_type;
                file
            }
            None => File::new(meta.owner_id, meta.file_name, content_type),
        };

        let size = self.storage.upload(&file, content).await?;

        file.set_size(size);

        let file = self.metadata.save(file).await?;

        if let Err(err) = self
            .refresh_collection
            .dispatch(RefreshCollections {
                owner_id: file.owner_id,
            })
            .await
        {
            tracing::error!(
                error = %err,
                account_id = %file.owner_id,
                file_id = %file.id,
                "failed to queue RefreshCollections job",
            );
        }

        Ok(file)
    }

    pub async fn update_tags<I>(&self, mut file: File, tags: I) -> Result<File, AddTagsError>
    where
        I: IntoIterator<Item = Tag>,
    {
        file.set_tags(tags);

        let file = self.metadata.save(file).await?;

        Ok(file)
    }

    pub async fn search(
        &self,
        owner_id: AccountId,
        query: impl AsRef<str>,
        paginate: Paginate,
    ) -> Result<Slice<File>, SearchError> {
        let filter = oxidrive_search::parse_query(query)?;
        let files = self.metadata.search(owner_id, filter, paginate).await?;
        Ok(files)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error(transparent)]
    DownloadFailed(#[from] DownloadFileError),
}

pub struct UploadMetadata {
    pub file_name: String,
    pub owner_id: AccountId,
}

#[derive(Debug, thiserror::Error)]
pub enum UploadError {
    #[error("failed to load file by name")]
    LoadFailed(#[from] ByNameError),
    #[error("failed to upload file content")]
    UploadFailed(#[from] UploadFileError),
    #[error("failed to save file metadata")]
    SaveMetadataFailed(#[from] SaveFileError),
}

#[derive(Debug, thiserror::Error)]
pub enum AddTagsError {
    #[error("failed to save file")]
    SaveFileFailed(#[from] SaveFileError),
}

#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error(transparent)]
    QueryParse(#[from] QueryParseError),
    #[error(transparent)]
    SearchFailed(#[from] file::SearchError),
}
