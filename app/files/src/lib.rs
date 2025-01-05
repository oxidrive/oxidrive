use std::{path::PathBuf, sync::Arc};

use bytes::Bytes;
use file::{
    ByNameError, DownloadFileError, FileContents, FileMetadata, FsFileContents, PgFileMetadata,
    SaveFileError, SqliteFileMetadata, UploadFileError,
};
use futures::{Stream, StreamExt, TryStreamExt};
use oxidrive_auth::account::{Account, AccountId};

pub struct FilesModule;

mod content_type;
pub mod file;

pub use file::{ContentStreamError, File};
use oxidrive_database::Database;
use serde::Deserialize;

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

        let file = match self
            .metadata
            .by_name(meta.owner_id, &meta.file_name)
            .await?
        {
            Some(mut file) => {
                file.content_type = content_type;
                file
            }
            None => File::create(meta.owner_id, meta.file_name, content_type),
        };

        self.contents
            .upload(&file, content.map_err(ContentStreamError::wrap).boxed())
            .await?;

        let file = self.metadata.save(file).await?;

        Ok(file)
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
