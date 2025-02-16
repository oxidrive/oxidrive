use axum::{extract::State, http::StatusCode, Json};
use oxidrive_accounts::{AccountService, ChangePasswordError, VerifyPasswordError};
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
    Json(UpdatePassword {
        current_password,
        new_password,
    }): Json<UpdatePassword>,
) -> ApiResult<()> {
    accounts.verify_password(&account, current_password).await?;
    accounts.change_password(&account, new_password).await?;
    Ok(())
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePassword {
    current_password: String,
    new_password: String,
}

impl From<VerifyPasswordError> for ApiError {
    fn from(err: VerifyPasswordError) -> Self {
        match err {
            VerifyPasswordError::LoadCredentials(err) => Self::new(err),
            VerifyPasswordError::InvalidPassword(err) => Self::new(err)
                .error("INVALID_CURRENT_PASSWORD")
                .status(StatusCode::BAD_REQUEST),
        }
    }
}

impl From<ChangePasswordError> for ApiError {
    fn from(err: ChangePasswordError) -> Self {
        match err {
            ChangePasswordError::LoadCredentials(err) => Self::new(err),
            ChangePasswordError::InvalidPassword(err) => Self::new(err)
                .error("INVALID_NEW_PASSWORD")
                .status(StatusCode::BAD_REQUEST),
            ChangePasswordError::SaveCredentials(err) => Self::new(err),
        }
    }
}
