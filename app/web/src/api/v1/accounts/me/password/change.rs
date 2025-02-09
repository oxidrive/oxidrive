use axum::{extract::State, http::StatusCode, Json};
use oxidrive_accounts::{AccountService, ChangePasswordError};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    api::error::{ApiError, ApiResult},
    session::CurrentUser,
};

#[utoipa::path(
    put,
    path = "/",
    operation_id = "change",
    responses((status = NO_CONTENT)),
    tag = "accounts",
)]
#[axum::debug_handler(state = crate::state::AppState)]
pub async fn handler(
    State(accounts): State<AccountService>,
    CurrentUser(account): CurrentUser,
    Json(UpdatePassword { password }): Json<UpdatePassword>,
) -> ApiResult<()> {
    accounts.change_password(&account, &password).await?;
    Ok(())
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePassword {
    password: String,
}

impl From<ChangePasswordError> for ApiError {
    fn from(err: ChangePasswordError) -> Self {
        match err {
            ChangePasswordError::LoadCredentials(err) => Self::new(err),
            ChangePasswordError::InvalidPassword(err) => Self::new(err)
                .error("INVALID_PASSWORD")
                .status(StatusCode::BAD_REQUEST),
            ChangePasswordError::SaveCredentials(err) => Self::new(err),
        }
    }
}
