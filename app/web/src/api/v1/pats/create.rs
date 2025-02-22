use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use oxidrive_accounts::{AccountService, pat::CreatePatError};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::{ToResponse, ToSchema};

use crate::{
    api::error::{ApiError, ApiResult},
    session::CurrentUser,
};

use super::PersonalAccessTokenData;

#[utoipa::path(
    post,
    path = "/",
    operation_id = "create",
    responses((status = CREATED, response = PersonalAccessTokenCreated)),
    tag = "pats",
)]
#[axum::debug_handler(state = crate::state::AppState)]
pub async fn handler(
    State(accounts): State<AccountService>,
    CurrentUser(account): CurrentUser,
    Json(CreatePersonalAccessToken { expires_at }): Json<CreatePersonalAccessToken>,
) -> ApiResult<PersonalAccessTokenCreated> {
    let (token, pat) = accounts
        .personal_access_tokens()
        .create(account.id, expires_at)
        .await?;

    Ok(PersonalAccessTokenCreated {
        token: token.to_string(),
        data: pat.into(),
    })
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePersonalAccessToken {
    #[serde(with = "time::serde::rfc3339::option")]
    expires_at: Option<OffsetDateTime>,
}

#[derive(Debug, Serialize, ToResponse)]
#[response(content_type = "application/json")]
#[response(examples(
    ("Token" = (value = json!({
        "token": "oxipat.9mcXeQKqacTb6aCk_N8umw",
        "id": "01950018-fbe9-7d93-a2b6-f0d1cac70708",
        "expires_at": "2025-02-14T16:16:32.225Z"
    }))),
))]
pub struct PersonalAccessTokenCreated {
    token: String,
    #[serde(flatten)]
    data: PersonalAccessTokenData,
}

impl IntoResponse for PersonalAccessTokenCreated {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

impl From<CreatePatError> for ApiError {
    fn from(err: CreatePatError) -> Self {
        match err {
            CreatePatError::SaveFailed(err) => Self::new(err),
            CreatePatError::InvalidExpirationDate(err) => Self::new(err)
                .status(StatusCode::BAD_REQUEST)
                .error("INVALID_EXPIRATION_DATE"),
        }
    }
}
