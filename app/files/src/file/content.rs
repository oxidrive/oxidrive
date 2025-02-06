use std::{fmt::Display, path::Path};

use bytes::Bytes;
use futures::{SinkExt, Stream, TryStreamExt};

use super::File;

// mod fs;

// pub use fs::*;

#[derive(Clone)]
pub struct FileStorage {
    service: opendal::Operator,
}

impl FileStorage {
    pub fn display_name(&self) -> impl Display {
        self.service.info().name().to_string()
    }

    pub async fn download(
        &self,
        file: &File,
    ) -> Result<Option<impl Stream<Item = Result<Bytes, impl std::error::Error>>>, DownloadFileError>
    {
        let path = path_for(file);

        if !self.service.exists(&path).await? {
            return Ok(None);
        }

        let reader = self.service.reader(&path).await?;

        reader
            .into_bytes_stream(..)
            .await
            .map(Some)
            .map_err(DownloadFileError)
    }

    pub async fn upload(
        &self,
        file: &File,
        content: impl Stream<Item = Result<Bytes, impl std::error::Error + Send + Sync + 'static>>
            + Unpin,
    ) -> Result<usize, UploadFileError> {
        let mut size = 0;

        let mut writer = self
            .service
            .writer(&path_for(file))
            .await?
            .into_bytes_sink();

        let mut content = content
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
            .inspect_ok(|bytes| {
                size += bytes.len();
            });

        writer.send_all(&mut content).await?;
        writer.close().await?;

        Ok(size)
    }
}

impl FileStorage {
    fn new(cfg: impl opendal::Configurator) -> Self {
        let service = opendal::Operator::from_config(cfg).unwrap().finish();

        Self { service }
    }

    pub fn memory() -> Self {
        Self::new(opendal::services::MemoryConfig::default())
    }

    pub fn file_system(root_dir: impl AsRef<Path>) -> Self {
        let mut cfg = opendal::services::FsConfig::default();
        cfg.root = Some(root_dir.as_ref().as_os_str().to_string_lossy().into());

        Self::new(cfg)
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct DownloadFileError(#[from] opendal::Error);

#[derive(Debug, thiserror::Error)]
pub enum UploadFileError {
    #[error(transparent)]
    ServiceError(#[from] opendal::Error),
    #[error("failed to write content to storage: {0}")]
    WriteFailed(#[from] std::io::Error),
}

fn path_for(file: &File) -> String {
    format!("/{}/{}", file.owner_id, file.name)
}

#[cfg(test)]
mod tests;
