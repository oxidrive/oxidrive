use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use oxidrive_auth::Auth;

use crate::Config;

#[derive(Clone)]
pub struct AppState {
    auth: Auth,
    key: Key,
}

impl AppState {
    pub fn new(cfg: Config, auth: Auth) -> Self {
        Self {
            auth,
            key: Key::from(cfg.secret_key.as_bytes()),
        }
    }
}

impl FromRef<AppState> for Auth {
    fn from_ref(state: &AppState) -> Self {
        state.auth.clone()
    }
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}
