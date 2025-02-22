use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use oxidrive_accounts::AccountService;
use oxidrive_authorization::Authorizer;
use oxidrive_files::{Files, collection::Collections};

use crate::Config;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub accounts: AccountService,
    pub authorizer: Authorizer,
    pub files: Files,
    pub collections: Collections,

    key: Key,
}

impl AppState {
    pub fn new(
        cfg: Config,
        accounts: AccountService,
        authorizer: Authorizer,
        files: Files,
        collections: Collections,
    ) -> Self {
        Self {
            accounts,
            authorizer,
            files,
            collections,
            key: Key::from(cfg.secret_key.as_bytes()),
        }
    }
}
