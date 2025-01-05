use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use oxidrive_auth::Auth;
use oxidrive_files::Files;
use oxidrive_tags::Tags;

use crate::Config;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub auth: Auth,
    pub files: Files,
    pub tags: Tags,

    key: Key,
}

impl AppState {
    pub fn new(cfg: Config, auth: Auth, files: Files, tags: Tags) -> Self {
        Self {
            auth,
            files,
            tags,
            key: Key::from(cfg.secret_key.as_bytes()),
        }
    }
}
