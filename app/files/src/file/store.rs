use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use globset::Glob;
use oxidrive_accounts::account::AccountId;
use oxidrive_domain::make_error_wrapper;
use oxidrive_paginate::{Paginate, Slice};
use oxidrive_search::{Filter, Values};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::Tag;

use super::{File, FileId, Tags};

mod pg;
mod sqlite;

pub use pg::*;
pub use sqlite::*;

make_error_wrapper!(AllOwnedByError);
make_error_wrapper!(AllOwnedByInError);
make_error_wrapper!(ByIdError);
make_error_wrapper!(ByNameError);
make_error_wrapper!(SaveFileError);
make_error_wrapper!(SearchError);

#[async_trait]
pub trait FileMetadata: Send + Sync + 'static {
    async fn all_owned_by(
        &self,
        owner_id: AccountId,
        paginate: Paginate,
    ) -> Result<Slice<File>, AllOwnedByError> {
        self.search(owner_id, Filter::All, paginate)
            .await
            .map_err(AllOwnedByError::wrap)
    }

    async fn all_owned_by_in(
        &self,
        owner_id: AccountId,
        ids: &[FileId],
        paginate: Paginate,
    ) -> Result<Slice<File>, AllOwnedByInError>;

    async fn by_id(&self, id: FileId) -> Result<Option<File>, ByIdError>;

    async fn by_owner_and_name(
        &self,
        owner_id: AccountId,
        file_name: &str,
    ) -> Result<Option<File>, ByNameError>;

    async fn save(&self, file: File) -> Result<File, SaveFileError>;

    async fn search(
        &self,
        owner_id: AccountId,
        filter: Filter,
        paginate: Paginate,
    ) -> Result<Slice<File>, SearchError>;
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
    async fn all_owned_by_in(
        &self,
        owner_id: AccountId,
        ids: &[FileId],
        params: Paginate,
    ) -> Result<Slice<File>, AllOwnedByInError> {
        let inner = self.inner.read().await;

        let files = inner
            .values()
            .filter(|f| f.owner_id == owner_id && ids.contains(&f.id));

        Ok(paginate(files, params))
    }

    async fn by_id(&self, id: FileId) -> Result<Option<File>, ByIdError> {
        let inner = self.inner.read().await;
        Ok(inner.get(&id).cloned())
    }

    async fn by_owner_and_name(
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

    async fn search(
        &self,
        owner_id: AccountId,
        filter: Filter,
        params: Paginate,
    ) -> Result<Slice<File>, SearchError> {
        let inner = self.inner.read().await;
        let mut filter = traverse(|_| true, filter);

        let files = inner
            .values()
            .filter(|f| f.owner_id == owner_id)
            .filter(|file| filter(&file.tags));

        Ok(paginate(files, params))
    }
}

type FilterFn = Box<dyn FnMut(&Tags) -> bool>;

fn traverse<F>(mut current: F, filter: Filter) -> FilterFn
where
    F: FnMut(&Tags) -> bool + Clone + 'static,
{
    match filter {
        Filter::All => Box::new(current),
        Filter::Tag { key, values } => Box::new(move |tags| {
            current(tags) && tags.values().any(|tag| tag_matches(tag, &key, &values))
        }),
        Filter::Op { lhs, op, rhs } => {
            let mut lhs = traverse(current.clone(), *lhs);
            let mut rhs = traverse(current, *rhs);
            Box::new(move |tags| match op {
                oxidrive_search::Op::And => lhs(tags) && rhs(tags),
                oxidrive_search::Op::Or => lhs(tags) || rhs(tags),
            })
        }
        Filter::Mod { modifier, inner } => {
            let mut inner = traverse(current, *inner);
            Box::new(move |tags| match modifier {
                oxidrive_search::Mod::Not => !inner(tags),
            })
        }
    }
}

fn tag_matches(tag: &Tag, key: &String, values: &Values) -> bool {
    let key_matches = &tag.key == key;

    // we only want to match on the key
    if values.is_empty() {
        return key_matches;
    }

    // the tag is key-only, so any value match is a failure
    if tag.value.is_none() {
        return false;
    }

    let value = tag.value.as_ref().unwrap();
    let values_str = values.to_string();

    // we match on the value itself, without globs
    if !values.has_matches() {
        return key_matches && value == &values_str;
    }

    let glob = Glob::new(&values_str).unwrap().compile_matcher();

    key_matches && glob.is_match(value)
}

fn paginate<'a, I>(files: I, params: Paginate) -> Slice<File>
where
    I: Iterator<Item = &'a File>,
{
    let (id, limit, is_forward) = match params {
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

    let files: Vec<File> = files
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
        Slice::new(files, next, None)
    } else {
        let previous = files.first().map(|f| f.id.to_string());
        Slice::new(files, None, previous)
    }
}

#[cfg(test)]
pub(crate) mod tests;
