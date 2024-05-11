use dioxus::prelude::*;

use crate::storage::{use_local_storage, UseLocalStorage};

const SESSION_KEY: &str = "oxidrive-auth-session";

pub type MaybeToken = Option<String>;

#[derive(Debug)]
pub struct CurrentUser {
    pub id: String,
    pub token: String,
}

pub fn use_current_user() -> Signal<Option<CurrentUser>> {
    use_context()
}

pub fn init() -> Signal<Option<CurrentUser>> {
    let token_storage = use_token_storage();
    use_context_provider::<Signal<Option<CurrentUser>>>(|| match token_storage.get() {
        Some(token) => Signal::new(Some(CurrentUser {
            id: "".into(),
            token,
        })),
        None => Signal::new(None),
    })
}

pub fn use_token_storage() -> UseLocalStorage<MaybeToken> {
    use_local_storage(SESSION_KEY, || None)
}

pub fn store_token(
    token_storage: &mut UseLocalStorage<MaybeToken>,
    mut current_user: Signal<Option<CurrentUser>>,
    token: String,
) {
    token_storage.set(Some(token.clone()));
    *current_user.write() = Some(CurrentUser {
        id: "".into(),
        token,
    });
}
