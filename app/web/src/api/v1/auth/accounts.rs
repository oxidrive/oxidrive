use axum::{extract::State, http::StatusCode, response::IntoResponse, Form, Json};
use oxidrive_auth::{
    account::{Account, AccountId},
    Auth, CreateAccountError,
};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[axum::debug_handler(state = AppState)]
pub async fn create(
    State(auth): State<Auth>,
    Form(CreateAccount { username, password }): Form<CreateAccount>,
) -> Result<Json<AccountInfo>, CreateError> {
    let account = auth.create_account(&username, &password).await?;
    Ok(Json(account.into()))
}

#[derive(Debug, Deserialize)]
pub struct CreateAccount {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct AccountInfo {
    id: AccountId,
    username: String,
}

impl From<Account> for AccountInfo {
    fn from(account: Account) -> Self {
        Self {
            id: account.id,
            username: account.username,
        }
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
