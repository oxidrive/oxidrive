use axum::Json;

use crate::{api::v1::accounts::AccountInfo, session::CurrentUser};

#[utoipa::path(get,
    path = "/",
    operation_id = "get",
    responses((status = OK, body = AccountInfo)),
    tag = "accounts",
)]
#[axum::debug_handler(state = crate::state::AppState)]
pub async fn handler(CurrentUser(account): CurrentUser) -> Json<AccountInfo> {
    Json(account.into())
}
