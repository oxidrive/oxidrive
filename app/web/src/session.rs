use std::fmt::Display;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use axum_extra::extract::{
    cookie::{Cookie, Expiration, Key},
    SignedCookieJar,
};
use oxidrive_auth::account::{Account, AccountId};

use crate::state::AppState;

const SESSION_COOKIE: &str = "oxidrive_session";

#[derive(Default)]
pub struct Session {
    pub account_id: AccountId,
}

impl Session {
    pub fn into_cookie(self) -> Cookie<'static> {
        Cookie::build((SESSION_COOKIE, self.to_string()))
            .path("/")
            .http_only(true)
            .expires(Expiration::Session)
            .build()
    }
}

impl From<Account> for Session {
    fn from(account: Account) -> Self {
        Self {
            account_id: account.id,
        }
    }
}

impl Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "account_id={}", self.account_id)
    }
}

impl TryFrom<String> for Session {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut data = Session::default();

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

impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
    Key: FromRef<S>,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = SignedCookieJar::<Key>::from_request_parts(parts, state)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let Some(cookie) = jar.get(SESSION_COOKIE) else {
            return Err(StatusCode::UNAUTHORIZED);
        };

        Session::try_from(cookie.value().to_string()).map_err(|_| StatusCode::UNAUTHORIZED)
    }
}

pub struct CurrentUser(pub Account);

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state).await?;

        let account = state
            .auth
            .accounts()
            .by_id(session.account_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?;
        Ok(Self(account))
    }
}
