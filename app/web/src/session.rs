use std::ops::Deref;

use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    http::request::Parts,
    response::IntoResponseParts,
};
use axum_extra::extract::{
    cookie::{Cookie, Key},
    SignedCookieJar,
};
use oxidrive_accounts::{account::Account, session::Session};

use crate::{api::error::ApiError, state::AppState};

pub const SESSION_COOKIE: &str = "oxidrive_session";

pub struct WebSession {
    session: Session,

    jar: SignedCookieJar,
}

impl WebSession {
    pub fn create(session: impl Into<Session>, jar: SignedCookieJar) -> Self {
        Self {
            session: session.into(),
            jar,
        }
    }

    pub fn clear(self) -> SignedCookieJar {
        let cookie = self.as_cookie();
        self.jar.remove(cookie)
    }

    fn as_cookie(&self) -> Cookie<'static> {
        Cookie::build((SESSION_COOKIE, self.session.id.to_string()))
            .path("/")
            .http_only(true)
            .expires(self.session.expires_at)
            .build()
    }
}

impl Deref for WebSession {
    type Target = Session;

    fn deref(&self) -> &Self::Target {
        &self.session
    }
}

impl IntoResponseParts for WebSession {
    type Error = <SignedCookieJar as IntoResponseParts>::Error;

    fn into_response_parts(
        self,
        res: axum::response::ResponseParts,
    ) -> Result<axum::response::ResponseParts, Self::Error> {
        let cookie = self.as_cookie();
        let jar = self.jar.add(cookie);
        jar.into_response_parts(res)
    }
}

impl FromRequestParts<AppState> for WebSession {
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

impl OptionalFromRequestParts<AppState> for WebSession {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Option<Self>, Self::Rejection> {
        let jar = SignedCookieJar::<Key>::from_request_parts(parts, state)
            .await
            .unwrap_or_else(|_| unreachable!("SignedCookieJar::from_request_parts is infallible"));

        let Some(cookie) = jar.get(SESSION_COOKIE) else {
            return Ok(None);
        };

        let session_id = cookie
            .value_trimmed()
            .parse()
            .map_err(|_| ApiError::unauthenticated())?;

        let Some(session) = state
            .accounts
            .sessions()
            .by_id(session_id)
            .await
            .map_err(ApiError::new)?
        else {
            return Ok(None);
        };

        Ok(Some(WebSession::create(session, jar)))
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
            <WebSession as OptionalFromRequestParts<AppState>>::from_request_parts(parts, state)
                .await?
        else {
            return Ok(None);
        };

        let Some(account) = state
            .accounts
            .accounts()
            .by_id(session.session.account_id)
            .await
            .map_err(ApiError::new)?
        else {
            return Ok(None);
        };

        Ok(Some(Self(account)))
    }
}
