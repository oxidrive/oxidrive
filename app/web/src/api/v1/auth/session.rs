use axum::{extract::State, http::StatusCode, response::IntoResponse, Form, Json};
use axum_extra::extract::cookie::SignedCookieJar;
use oxidrive_auth::{account::AccountId, login::AuthenticationFailed, Auth};
use serde::{Deserialize, Serialize};

use crate::{
    session::{CurrentUser, Session},
    state::AppState,
};

#[axum::debug_handler(state = AppState)]
pub async fn get(CurrentUser(account): CurrentUser) -> Result<Json<SessionInfo>, StatusCode> {
    Ok(Json(SessionInfo {
        account_id: account.id,
        username: account.username,
    }))
}

#[derive(Debug, Serialize)]
pub struct SessionInfo {
    account_id: AccountId,
    username: String,
}

#[axum::debug_handler(state = AppState)]
pub async fn create(
    State(auth): State<Auth>,
    jar: SignedCookieJar,
    Form(creds): Form<SessionCredentials>,
) -> Result<SessionResponse, CreateSessionError> {
    let account = match creds {
        SessionCredentials::Password { username, password } => {
            auth.login().password(&username, &password).await?
        }
    };

    let jar = jar.add(Session::from(account).into_cookie());

    Ok(SessionResponse { jar })
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum SessionCredentials {
    Password { username: String, password: String },
}

#[derive(Debug)]
pub struct SessionResponse {
    jar: SignedCookieJar,
}

impl IntoResponse for SessionResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, self.jar).into_response()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CreateSessionError {
    #[error(transparent)]
    Password(#[from] AuthenticationFailed),
}

impl IntoResponse for CreateSessionError {
    fn into_response(self) -> axum::response::Response {
        match self {
            CreateSessionError::Password(AuthenticationFailed::Unauthorized) => {
                StatusCode::UNAUTHORIZED.into_response()
            }

            CreateSessionError::Password(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")).into_response()
            }
        }
    }
}
