use axum::Json;
use serde::Serialize;
use uuid::Uuid;

use crate::{session::CurrentUser, state::AppState};

#[axum::debug_handler(state = AppState)]
pub async fn handler(CurrentUser(account): CurrentUser) -> Json<SessionInfo> {
    Json(SessionInfo {
        account_id: account.id.as_uuid(),
        username: account.username,
    })
}

#[derive(Debug, Serialize)]
pub struct SessionInfo {
    account_id: Uuid,
    username: String,
}
