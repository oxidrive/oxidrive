use axum::Json;
use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::session::{CurrentUser, WebSession};

#[axum::debug_handler(state = crate::state::AppState)]
pub async fn handler(session: WebSession, CurrentUser(account): CurrentUser) -> Json<SessionInfo> {
    Json(SessionInfo {
        session_id: session.id.as_uuid(),
        account_id: session.account_id.as_uuid(),
        username: account.username,
        expires_at: session.expires_at,
    })
}

#[derive(Debug, Serialize)]
pub struct SessionInfo {
    session_id: Uuid,
    account_id: Uuid,
    username: String,
    #[serde(with = "time::serde::rfc3339")]
    expires_at: OffsetDateTime,
}
