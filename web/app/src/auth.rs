use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use oxidrive_api::models::Session;

use crate::{
    api::use_oxidrive_api,
    storage::{use_local_storage, UseLocalStorage},
};

const SESSION_KEY: &str = "oxidrive-auth-session";

pub type SessionStorage = UseLocalStorage<Option<Session>>;

#[derive(Debug)]
pub struct CurrentUser {
    pub id: String,
    pub token: String,
}

pub fn use_current_user() -> Signal<Option<CurrentUser>> {
    use_context()
}

pub fn init() -> Signal<Option<CurrentUser>> {
    let mut api = use_oxidrive_api();
    let token_storage = use_session_storage();

    use_context_provider::<Signal<Option<CurrentUser>>>(move || {
        let mut api = api.write();

        match token_storage.get() {
            Some(Session { token, expires_at }) => {
                let expires_at = DateTime::parse_from_rfc3339(&expires_at)
                    .expect("failed to parse session.expires_at to an RFC3339 timestamp");

                if expires_at <= Utc::now() {
                    return Signal::new(None);
                }

                api.set_token(token.clone());

                Signal::new(Some(CurrentUser {
                    id: "".into(),
                    token,
                }))
            }
            None => {
                api.remove_token();

                Signal::new(None)
            }
        }
    })
}

pub fn use_session_storage() -> SessionStorage {
    use_local_storage(SESSION_KEY, || None)
}

pub fn store_token(
    token_storage: &mut SessionStorage,
    mut current_user: Signal<Option<CurrentUser>>,
    session: Session,
) {
    token_storage.set(Some(session.clone()));
    *current_user.write() = Some(CurrentUser {
        id: "".into(),
        token: session.token,
    });
}
