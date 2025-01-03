use std::fmt::Display;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Form, Json};
use axum_extra::extract::cookie::{Cookie, Expiration, SignedCookieJar};
use oxidrive_auth::{
    account::{Account, AccountId},
    login::AuthenticationFailed,
    Auth,
};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[axum::debug_handler(state = AppState)]
pub async fn get(
    State(auth): State<Auth>,
    jar: SignedCookieJar,
) -> Result<Json<SessionInfo>, StatusCode> {
    let Some(cookie) = jar.get(SESSION_COOKIE) else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let SessionData { account_id } =
        SessionData::try_from(cookie.value().to_string()).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let account = auth
        .accounts()
        .by_id(account_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

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

    let jar = jar.add(session_cookie(account));

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
            CreateSessionError::Password(AuthenticationFailed) => {
                StatusCode::UNAUTHORIZED.into_response()
            }
        }
    }
}

const SESSION_COOKIE: &str = "oxidrive_session";

fn session_cookie(account: Account) -> Cookie<'static> {
    Cookie::build((SESSION_COOKIE, SessionData::from(account).to_string()))
        .path("/")
        .http_only(true)
        .expires(Expiration::Session)
        .build()
}

#[derive(Default)]
struct SessionData {
    account_id: AccountId,
}

impl From<Account> for SessionData {
    fn from(account: Account) -> Self {
        Self {
            account_id: account.id,
        }
    }
}

impl Display for SessionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "account_id={}", self.account_id)
    }
}

impl TryFrom<String> for SessionData {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut data = SessionData::default();

        for pairs in value.clone().split('&').map(|s| s.trim()) {
            let mut pair = pairs.split('=');

            let key = pair.next().ok_or_else(|| value.clone())?;
            let val = pair.next().ok_or_else(|| value.clone())?;

            match key {
                "account_id" => {
                    data.account_id = val.parse().map_err(|_| value.clone())?;
                }

                _ => {
                    return Err(value);
                }
            }
        }

        Ok(data)
    }
}
