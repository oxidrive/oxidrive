use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use oxidrive_auth::Auth;
use oxidrive_files::{collection::Collections, Files};

use crate::Config;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub auth: Auth,
    pub files: Files,
    pub collections: Collections,

    key: Key,
}

impl AppState {
    pub fn new(cfg: Config, auth: Auth, files: Files, collections: Collections) -> Self {
        Self {
            auth,
            files,
            collections,
            key: Key::from(cfg.secret_key.as_bytes()),
        }
    }
}
