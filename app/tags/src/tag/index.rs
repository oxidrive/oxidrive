use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use async_trait::async_trait;
use oxidrive_domain::make_error_wrapper;
use oxidrive_files::{file::FileId, File};
use tokio::sync::RwLock;

use super::Tag;

pub use pg::*;
pub use sqlite::*;

mod pg;
mod sqlite;

make_error_wrapper!(IndexTagError);
make_error_wrapper!(ForFileError);

#[async_trait]
pub trait TagIndex: Send + Sync + 'static {
    async fn index(&self, file: &File, tags: Vec<Tag>) -> Result<(), IndexTagError>;
    async fn for_file(&self, file_id: FileId) -> Result<Vec<Tag>, ForFileError>;
}

#[derive(Default)]
pub struct InMemoryTagIndex {
    inner: Arc<RwLock<HashMap<Tag, HashMap<FileId, File>>>>,
}

#[async_trait]
impl TagIndex for InMemoryTagIndex {
    async fn index(&self, file: &File, tags: Vec<Tag>) -> Result<(), IndexTagError> {
        let mut inner = self.inner.write().await;

        for tag in tags {
            let files = inner.entry(tag).or_default();
            files.insert(file.id, file.clone());
        }

        Ok(())
    }

    async fn for_file(&self, file_id: FileId) -> Result<Vec<Tag>, ForFileError> {
        let inner = self.inner.read().await;

        let files = inner
            .iter()
            .filter(|(_, files)| files.contains_key(&file_id))
            .map(|(tag, _)| tag)
            .cloned()
            .collect();
        Ok(files)
    }
}

#[cfg(test)]
mod tests;
