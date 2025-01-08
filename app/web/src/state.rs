use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use oxidrive_auth::Auth;
use oxidrive_files::Files;

use crate::Config;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub auth: Auth,
    pub files: Files,

    key: Key,
}

impl AppState {
    pub fn new(cfg: Config, auth: Auth, files: Files) -> Self {
        Self {
            auth,
            files,
            key: Key::from(cfg.secret_key.as_bytes()),
        }
    }
}
