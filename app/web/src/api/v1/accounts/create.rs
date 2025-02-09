use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use oxidrive_accounts::{AccountService, CreateAccountError};
use serde::Deserialize;
use utoipa::{ToResponse, ToSchema};

use crate::api::error::{ApiError, ApiResult};

use super::AccountInfo;

#[utoipa::path(
    post,
    path = "/",
    operation_id = "create",
    responses((status = CREATED, response = AccountCreated)),
    tag = "accounts",
)]
#[axum::debug_handler(state = crate::state::AppState)]
pub async fn handler(
    State(accounts): State<AccountService>,
    Json(CreateAccount { username, password }): Json<CreateAccount>,
) -> ApiResult<AccountCreated> {
    let account = accounts.create_account(&username, &password).await?;
    Ok(AccountCreated(account.into()))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAccount {
    username: String,
    password: String,
}

#[derive(Debug, ToResponse)]
#[response(content_type = "application/json")]
pub struct AccountCreated(AccountInfo);

impl IntoResponse for AccountCreated {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, Json(self.0)).into_response()
    }
}

impl From<CreateAccountError> for ApiError {
    fn from(err: CreateAccountError) -> Self {
        match err {
            CreateAccountError::Load(err) => Self::new(err),
            CreateAccountError::AlreadyExists => Self::new("account already exists")
                .error("ALREADY_EXISTS")
                .status(StatusCode::CONFLICT),
            CreateAccountError::SaveAccount(err) => Self::new(err),
            CreateAccountError::InvalidPassword(err) => Self::new(err)
                .error("INVALID_PASSWORD")
                .status(StatusCode::BAD_REQUEST),
            CreateAccountError::SaveCredentials(err) => Self::new(err),
        }
    }
}
