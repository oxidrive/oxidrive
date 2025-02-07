use std::fmt::Display;

use axum::{
    extract::{FromRef, FromRequestParts, OptionalFromRequestParts},
    http::request::Parts,
    response::IntoResponseParts,
};
use axum_extra::extract::{
    cookie::{Cookie, Expiration, Key},
    SignedCookieJar,
};
use oxidrive_accounts::account::{Account, AccountId};

use crate::{api::error::ApiError, state::AppState};

pub const SESSION_COOKIE: &str = "oxidrive_session";

#[derive(Debug)]
pub struct Session {
    pub data: SessionData,

    jar: SignedCookieJar,
}

impl Session {
    pub fn create(data: impl Into<SessionData>, jar: SignedCookieJar) -> Self {
        Self {
            data: data.into(),
            jar,
        }
    }

    pub fn clear(self) -> impl IntoResponseParts {
        self.jar.remove(self.data.into_cookie())
    }
}

impl IntoResponseParts for Session {
    type Error = <SignedCookieJar as IntoResponseParts>::Error;

    fn into_response_parts(
        self,
        res: axum::response::ResponseParts,
    ) -> Result<axum::response::ResponseParts, Self::Error> {
        let jar = self.jar.add(self.data.into_cookie());
        jar.into_response_parts(res)
    }
}

#[derive(Default, Debug)]
pub struct SessionData {
    pub account_id: AccountId,
}

impl SessionData {
    pub fn into_cookie(self) -> Cookie<'static> {
        Cookie::build((SESSION_COOKIE, self.to_string()))
            .path("/")
            .http_only(true)
            .expires(Expiration::Session)
            .build()
    }
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

impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
    Key: FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        <Self as OptionalFromRequestParts<S>>::from_request_parts(parts, state)
            .await?
            .ok_or_else(ApiError::unauthenticated)
    }
}

impl<S> OptionalFromRequestParts<S> for Session
where
    S: Send + Sync,
    Key: FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        let jar = SignedCookieJar::<Key>::from_request_parts(parts, state)
            .await
            .unwrap_or_else(|_| unreachable!("SignedCookieJar::from_request_parts is infallible"));

        let Some(cookie) = jar.get(SESSION_COOKIE) else {
            return Ok(None);
        };

        let data = SessionData::try_from(cookie.value().to_string())
            .map_err(|_| ApiError::unauthenticated())?;
        Ok(Some(Session::create(data, jar)))
    }
}

#[derive(Debug)]
pub struct CurrentUser(pub Account);

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        <Self as OptionalFromRequestParts<AppState>>::from_request_parts(parts, state)
            .await?
            .ok_or_else(ApiError::unauthenticated)
    }
}

impl OptionalFromRequestParts<AppState> for CurrentUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Option<Self>, Self::Rejection> {
        let Some(session) =
            <Session as OptionalFromRequestParts<AppState>>::from_request_parts(parts, state)
                .await?
        else {
            return Ok(None);
        };

        let Some(account) = state
            .auth
            .accounts()
            .by_id(session.data.account_id)
            .await
            .map_err(ApiError::new)?
        else {
            return Ok(None);
        };

        Ok(Some(Self(account)))
    }
}
