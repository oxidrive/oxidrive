use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{de::DeserializeOwned, Serialize};

pub fn use_local_storage<T: Serialize + DeserializeOwned + Default + 'static>(
    key: impl ToString,
    init: impl FnOnce() -> T,
) -> UseLocalStorage<T> {
    let state = use_signal(move || {
        let key = key.to_string();
        let value = LocalStorage::get(key.as_str()).ok().unwrap_or_else(init);
        StorageEntry { key, value }
    });

    UseLocalStorage { inner: state }
}

struct StorageEntry<T> {
    key: String,
    value: T,
}

/// Storage that persists across application reloads
pub struct UseLocalStorage<T: 'static> {
    inner: Signal<StorageEntry<T>>,
}

impl<T> Clone for UseLocalStorage<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for UseLocalStorage<T> {}

impl<T: Serialize + DeserializeOwned + Clone + 'static> UseLocalStorage<T> {
    pub fn get(&self) -> T {
        self.inner.read().value.clone()
    }

    pub fn set(&mut self, value: T) {
        let mut inner = self.inner.write();
        LocalStorage::set(inner.key.as_str(), &value).unwrap();
        inner.value = value;
    }
}
