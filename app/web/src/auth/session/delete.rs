use axum::extract::{Query, State};
use axum::http::header::REFERER;
use axum::http::{self, StatusCode, Uri};
use axum::response::Redirect;
use axum::{http::HeaderMap, response::IntoResponse};
use axum_extra::extract::SignedCookieJar;
use oxidrive_accounts::AccountService;
use serde::Deserialize;
use url::Url;

use crate::api::error::ApiError;
use crate::session::WebSession;

#[axum::debug_handler(state = crate::state::AppState)]
pub async fn handler(
    State(accounts): State<AccountService>,
    headers: HeaderMap,
    session: WebSession,
    Query(query): Query<DeleteQuery>,
) -> Result<SessionDeleted, DeleteSessionError> {
    // TODO: replace with this as soon as a new docs.rs/headers release is cut
    // let referer = headers
    //     .typed_get::<Referer>()
    //     .map(|referer| referer.to_string());

    let referer = headers
        .get(REFERER)
        .and_then(|referer| referer.to_str().ok())
        .and_then(|referer| Url::parse(referer).ok());

    let error = DeleteSessionError::new(referer);

    let redirect_to = query
        .redirect_to()
        .map_err(|err| ApiError::new(err).status(StatusCode::BAD_REQUEST))
        .map_err(|err| error.clone().wrap(err))?;

    if let Some(ref redirect_to) = redirect_to {
        if redirect_to.host().is_some() {
            return Err(error.wrap(
                ApiError::new("redirect_to must be a relative URI").status(StatusCode::BAD_REQUEST),
            ));
        }
    }

    if let Err(err) = accounts.sessions().delete(session.id).await {
        return Err(error.wrap(err.into()));
    }

    let jar = session.clear();

    Ok(SessionDeleted { jar, redirect_to })
}

#[derive(Debug, Deserialize)]
pub struct DeleteQuery {
    redirect_to: Option<String>,
}

impl DeleteQuery {
    fn redirect_to(&self) -> Result<Option<Uri>, http::uri::InvalidUri> {
        let Some(redirect_to) = &self.redirect_to else {
            return Ok(None);
        };

        let redirect_to = redirect_to.parse::<Uri>()?;
        Ok(Some(redirect_to))
    }
}

pub struct SessionDeleted {
    jar: SignedCookieJar,
    redirect_to: Option<Uri>,
}

impl IntoResponse for SessionDeleted {
    fn into_response(self) -> axum::response::Response {
        let redirect_to = self.redirect_to.as_ref().and_then(|r| r.path_and_query());

        match redirect_to {
            Some(redirect_to) => (self.jar, Redirect::to(redirect_to.as_str())).into_response(),
            None => (self.jar, StatusCode::NO_CONTENT).into_response(),
        }
    }
}

#[derive(Clone)]
pub struct DeleteSessionError {
    redirect_to: Option<Url>,
    error: ApiError,
}

impl From<oxidrive_accounts::session::DeleteSessionError> for ApiError {
    fn from(err: oxidrive_accounts::session::DeleteSessionError) -> Self {
        ApiError::new(err)
    }
}

impl DeleteSessionError {
    fn new(redirect_to: Option<Url>) -> Self {
        Self {
            redirect_to,
            error: ApiError::new("unknown"),
        }
    }

    fn wrap(mut self, error: ApiError) -> Self {
        self.error = error;
        self
    }
}

impl IntoResponse for DeleteSessionError {
    fn into_response(mut self) -> axum::response::Response {
        let Some(redirect_to) = self.redirect_to.as_mut() else {
            return self.error.into_response();
        };

        let error = self.error.into_body();

        {
            let mut query = redirect_to.query_pairs_mut();
            query.append_pair("error", &error.error);
            query.append_pair("error_description", &error.message);
        }

        Redirect::to(redirect_to.as_str()).into_response()
    }
}
