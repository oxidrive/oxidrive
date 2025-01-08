use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use futures::{stream::BoxStream, StreamExt, TryStreamExt};
use oxidrive_auth::account::AccountId;
use oxidrive_domain::make_error_wrapper;
use tokio::sync::RwLock;

use super::File;

mod fs;

pub use fs::*;

make_error_wrapper!(ContentStreamError);
make_error_wrapper!(DownloadFileError);
make_error_wrapper!(UploadFileError);

#[async_trait]
pub trait FileContents: Send + Sync + 'static {
    async fn download(
        &self,
        owner_id: AccountId,
        file_name: &str,
    ) -> Result<Option<BoxStream<'static, Result<Bytes, ContentStreamError>>>, DownloadFileError>;

    async fn upload(
        &self,
        file: &File,
        content: BoxStream<'_, Result<Bytes, ContentStreamError>>,
    ) -> Result<usize, UploadFileError>;
}

#[derive(Clone, Default)]
pub struct InMemoryFileContents {
    inner: Arc<RwLock<HashMap<String, Bytes>>>,
}

impl<const N: usize> From<[(String, Bytes); N]> for InMemoryFileContents {
    fn from(contents: [(String, Bytes); N]) -> Self {
        let accounts = HashMap::from(contents);
        Self {
            inner: Arc::new(RwLock::new(accounts)),
        }
    }
}

#[async_trait]
impl FileContents for InMemoryFileContents {
    async fn download(
        &self,
        owner_id: AccountId,
        file_name: &str,
    ) -> Result<Option<BoxStream<'static, Result<Bytes, ContentStreamError>>>, DownloadFileError>
    {
        let inner = self.inner.read().await;
        let Some(content) = inner.get(&path_for(owner_id, file_name)).cloned() else {
            return Ok(None);
        };

        Ok(Some(
            futures::stream::once(async move { Ok(content) }).boxed(),
        ))
    }

    async fn upload(
        &self,
        file: &File,
        content: BoxStream<'_, Result<Bytes, ContentStreamError>>,
    ) -> Result<usize, UploadFileError> {
        let mut inner = self.inner.write().await;

        let content: BytesMut = content.try_collect().await.map_err(UploadFileError::wrap)?;
        let size = content.len();

        inner.insert(path_for(file.owner_id, &file.name), content.freeze());

        Ok(size)
    }
}

fn path_for(owner_id: AccountId, file_name: &str) -> String {
    format!("{owner_id}/{file_name}")
}

#[cfg(test)]
mod tests;
