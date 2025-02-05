use axum::{extract::State, http::StatusCode, response::IntoResponse, Form, Json};
use oxidrive_accounts::{Auth, CreateAccountError};
use serde::Deserialize;
use utoipa::{ToResponse, ToSchema};

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
    State(auth): State<Auth>,
    Form(CreateAccount { username, password }): Form<CreateAccount>,
) -> Result<AccountCreated, CreateError> {
    let account = auth.create_account(&username, &password).await?;
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

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct CreateError(#[from] CreateAccountError);

impl IntoResponse for CreateError {
    fn into_response(self) -> axum::response::Response {
        match self.0 {
            CreateAccountError::Load(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")).into_response()
            }
            CreateAccountError::AlreadyExists => {
                (StatusCode::CONFLICT, format!("{}", self.0)).into_response()
            }
            CreateAccountError::SaveAccount(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")).into_response()
            }
            CreateAccountError::InvalidPassword(err) => {
                (StatusCode::BAD_REQUEST, format!("{err}")).into_response()
            }
            CreateAccountError::SaveCredentials(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")).into_response()
            }
        }
    }
}
