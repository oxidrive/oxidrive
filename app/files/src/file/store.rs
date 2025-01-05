use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use oxidrive_auth::account::AccountId;
use oxidrive_domain::make_error_wrapper;
use oxidrive_paginate::{Paginate, Slice};
use tokio::sync::RwLock;
use uuid::Uuid;

use super::{File, FileId};

mod pg;
mod sqlite;

pub use pg::*;
pub use sqlite::*;

make_error_wrapper!(AllOwnedByError);
make_error_wrapper!(ByNameError);
make_error_wrapper!(SaveFileError);

#[async_trait]
pub trait FileMetadata: Send + Sync + 'static {
    async fn all_owned_by(
        &self,
        owner_id: AccountId,
        paginate: Paginate,
    ) -> Result<Slice<File>, AllOwnedByError>;

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
    async fn all_owned_by(
        &self,
        owner_id: AccountId,
        paginate: Paginate,
    ) -> Result<Slice<File>, AllOwnedByError> {
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

        let mut files: Vec<File> = inner
            .values()
            .filter(|f| f.owner_id == owner_id)
            .filter(|f| {
                if is_forward {
                    f.id.to_string() > id
                } else {
                    f.id.to_string() < id
                }
            })
            .take(limit)
            .cloned()
            .collect();

        if is_forward {
            let next = files.last().map(|f| f.id.to_string());
            Ok(Slice::new(files, next, None))
        } else {
            files.reverse();
            let previous = files.last().map(|f| f.id.to_string());
            Ok(Slice::new(files, None, previous))
        }
    }

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
