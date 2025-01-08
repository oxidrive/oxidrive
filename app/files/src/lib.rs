use std::{path::PathBuf, sync::Arc};

use bytes::Bytes;
use file::{
    ByIdError, ByNameError, DownloadFileError, FileContents, FileId, FileMetadata, FsFileContents,
    PgFileMetadata, SaveFileError, SqliteFileMetadata, UploadFileError,
};
use futures::{Stream, StreamExt, TryStreamExt};
use oxidrive_auth::account::{Account, AccountId};
use oxidrive_database::Database;
use oxidrive_paginate::{Paginate, Slice};
use oxidrive_search::QueryParseError;
use serde::Deserialize;

pub struct FilesModule;

mod content_type;
pub mod file;
pub mod tag;

pub use file::{ContentStreamError, File};
pub use tag::Tag;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "provider")]
pub enum Config {
    #[serde(alias = "fs")]
    FileSystem { root_folder_path: PathBuf },
}

#[derive(Clone)]
pub struct Files {
    metadata: Arc<dyn FileMetadata>,
    contents: Arc<dyn FileContents>,
}

impl Files {
    pub fn new(files: Arc<dyn FileMetadata>, contents: Arc<dyn FileContents>) -> Self {
        Self {
            metadata: files,
            contents,
        }
    }

    pub fn metadata(&self) -> &dyn FileMetadata {
        self.metadata.as_ref()
    }

    pub async fn download(
        &self,
        owner: &Account,
        file_name: &str,
    ) -> Result<
        Option<(
            File,
            impl Stream<Item = Result<Bytes, ContentStreamError>> + 'static,
        )>,
        DownloadError,
    > {
        let Some(file) = self.metadata.by_name(owner.id, file_name).await? else {
            return Ok(None);
        };

        let Some(body) = self.contents.download(owner.id, file_name).await? else {
            return Ok(None);
        };

        Ok(Some((file, body)))
    }

    pub async fn upload<C, E>(&self, meta: UploadMetadata, content: C) -> Result<File, UploadError>
    where
        C: Stream<Item = Result<Bytes, E>> + Unpin + Send,
        E: std::error::Error + Send + Sync + 'static,
    {
        let (content, content_type) = content_type::detect_from_stream(content).await;

        let mut file = match self
            .metadata
            .by_name(meta.owner_id, &meta.file_name)
            .await?
        {
            Some(mut file) => {
                file.content_type = content_type;
                file
            }
            None => File::new(meta.owner_id, meta.file_name, content_type),
        };

        let size = self
            .contents
            .upload(&file, content.map_err(ContentStreamError::wrap).boxed())
            .await?;

        file.size = size;

        let file = self.metadata.save(file).await?;

        Ok(file)
    }

    pub async fn add_tags<I>(
        &self,
        owner_id: AccountId,
        file_id: FileId,
        tags: I,
    ) -> Result<(), AddTagsError>
    where
        I: IntoIterator<Item = Tag>,
    {
        let Some(mut file) = self.metadata.by_id(owner_id, file_id).await? else {
            return Err(AddTagsError::FileNotFound);
        };

        file.add_tags(tags);

        self.metadata.save(file).await?;

        Ok(())
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
    #[error("failed to load file by name")]
    LoadFailed(#[from] ByNameError),
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
    #[error("failed to load file by id")]
    LoadFailed(#[from] ByIdError),
    #[error("file does not exist")]
    FileNotFound,
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

impl app::Module for FilesModule {
    fn mount(self: Box<Self>, c: &mut app::di::Context) {
        c.bind(metadata);
        c.bind(contents);
        c.bind(Files::new);
    }
}

fn metadata(database: Database) -> Arc<dyn FileMetadata> {
    match database {
        Database::Sqlite(pool) => Arc::new(SqliteFileMetadata::new(pool)),
        Database::Pg(pool) => Arc::new(PgFileMetadata::new(pool)),
    }
}

fn contents(cfg: Config) -> Arc<dyn FileContents> {
    match cfg {
        Config::FileSystem { root_folder_path } => Arc::new(FsFileContents::new(root_folder_path)),
    }
}
