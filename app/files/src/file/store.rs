use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use oxidrive_auth::account::AccountId;
use oxidrive_domain::make_error_wrapper;
use tokio::sync::RwLock;

use super::{File, FileId};

mod pg;
mod sqlite;

pub use pg::*;
pub use sqlite::*;

make_error_wrapper!(ByNameError);
make_error_wrapper!(SaveFileError);

#[async_trait]
pub trait FileMetadata: Send + Sync + 'static {
    async fn by_name(
        &self,
        owner_id: AccountId,
        file_name: &str,
    ) -> Result<Option<File>, ByNameError>;

    async fn save(&self, file: File) -> Result<File, SaveFileError>;
}

#[derive(Clone, Default)]
pub struct InMemoryFileMetadata {
    inner: Arc<RwLock<HashMap<FileId, File>>>,
}

impl<const N: usize> From<[File; N]> for InMemoryFileMetadata {
    fn from(files: [File; N]) -> Self {
        let files = HashMap::from_iter(files.into_iter().map(|f| (f.id, f)));
        Self {
            inner: Arc::new(RwLock::new(files)),
        }
    }
}

#[async_trait]
impl FileMetadata for InMemoryFileMetadata {
    async fn by_name(
        &self,
        owner_id: AccountId,
        file_name: &str,
    ) -> Result<Option<File>, ByNameError> {
        let inner = self.inner.read().await;
        Ok(inner
            .values()
            .find(|f| f.owner_id == owner_id && f.name == file_name)
            .cloned())
    }

    async fn save(&self, file: File) -> Result<File, SaveFileError> {
        let mut inner = self.inner.write().await;
        inner.insert(file.id, file.clone());
        Ok(file)
    }
}

#[cfg(test)]
mod tests;
