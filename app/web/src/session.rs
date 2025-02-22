use std::ops::Deref;

use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    http::request::Parts,
    response::IntoResponseParts,
};
use axum_extra::{
    TypedHeader,
    extract::{
        SignedCookieJar,
        cookie::{Cookie, Key},
    },
    headers::{Authorization, authorization::Bearer},
};
use oxidrive_accounts::{
    account::{Account, AccountId},
    pat::PersonalAccessToken,
    session::Session,
};

use crate::{
    api::error::{ApiError, ApiResult},
    state::AppState,
};

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

        let Some(session_id) = cookie.value_trimmed().parse().ok() else {
            return Ok(None);
        };

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
enum BearerToken {
    Pat(PersonalAccessToken),
}

impl FromRequestParts<AppState> for BearerToken {
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

impl OptionalFromRequestParts<AppState> for BearerToken {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Option<Self>, Self::Rejection> {
        let Some(TypedHeader(Authorization(header))) = <TypedHeader<Authorization<Bearer>> as OptionalFromRequestParts<
            AppState,
        >>::from_request_parts(parts, state)
        .await
        .map_err(ApiError::new)?
        else {
            return Ok(None);
        };

        let token = header
            .token()
            .parse()
            .map_err(|err| ApiError::unauthenticated().message(err))?;

        let Some(token) = state
            .accounts
            .personal_access_tokens()
            .by_token(token)
            .await
            .map_err(ApiError::new)?
        else {
            return Ok(None);
        };

        Ok(Some(Self::Pat(token)))
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
        let Some(account_id) = extract_account_id(parts, state).await? else {
            return Ok(None);
        };

        let Some(account) = state
            .accounts
            .accounts()
            .by_id(account_id)
            .await
            .map_err(ApiError::new)?
        else {
            return Ok(None);
        };

        Ok(Some(Self(account)))
    }
}

async fn extract_account_id(parts: &mut Parts, state: &AppState) -> ApiResult<Option<AccountId>> {
    if let Some(session) =
        <WebSession as OptionalFromRequestParts<AppState>>::from_request_parts(parts, state).await?
    {
        return Ok(Some(session.account_id));
    }

    if let Some(token) =
        <BearerToken as OptionalFromRequestParts<AppState>>::from_request_parts(parts, state)
            .await?
    {
        return Ok(Some(match token {
            BearerToken::Pat(pat) => pat.account_id,
        }));
    }

    Ok(None)
}
