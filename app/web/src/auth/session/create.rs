use axum::{
    extract::{Query, State},
    http::{self, header::REFERER, HeaderMap, StatusCode, Uri},
    response::{IntoResponse, Redirect},
    Form,
};
use axum_extra::extract::cookie::SignedCookieJar;
use oxidrive_accounts::{account::VerifyCreds, AccountService};
use serde::Deserialize;
use url::Url;

use crate::{api::error::ApiError, session::WebSession};

#[axum::debug_handler(state = crate::state::AppState)]
pub async fn handler(
    State(auth): State<AccountService>,
    jar: SignedCookieJar,
    Query(query): Query<CreateQuery>,
    headers: HeaderMap,
    Form(creds): Form<SessionCredentials>,
) -> Result<SessionCreated, CreateSessionError> {
    // TODO: replace with this as soon as a new docs.rs/headers release is cut
    // let referer = headers
    //     .typed_get::<Referer>()
    //     .and_then(|referer| Url::parse(&referer.to_string()).ok());
    let referer = headers
        .get(REFERER)
        .and_then(|referer| referer.to_str().ok())
        .and_then(|referer| Url::parse(referer).ok());

    let error = CreateSessionError::new(referer);

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

    let (account, creds) = match creds {
        SessionCredentials::Password { username, password } => {
            let Some(account) = auth
                .accounts()
                .by_username(&username)
                .await
                .map_err(ApiError::new)
                .map_err(|err| error.clone().wrap(err))?
            else {
                return Err(error.wrap(ApiError::unauthenticated()));
            };

            (account, VerifyCreds::Password(password))
        }
    };

    let session = auth
        .sessions()
        .create(&account, creds)
        .await
        .map_err(ApiError::from)
        .map_err(move |err| error.wrap(err))?;

    let session = WebSession::create(session, jar);

    Ok(SessionCreated {
        session,
        redirect_to,
    })
}

#[derive(Debug, Deserialize)]
pub struct CreateQuery {
    redirect_to: Option<String>,
}

impl CreateQuery {
    fn redirect_to(&self) -> Result<Option<Uri>, http::uri::InvalidUri> {
        let Some(redirect_to) = &self.redirect_to else {
            return Ok(None);
        };

        let redirect_to = redirect_to.parse::<Uri>()?;
        Ok(Some(redirect_to))
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum SessionCredentials {
    Password { username: String, password: String },
}

pub struct SessionCreated {
    session: WebSession,
    redirect_to: Option<Uri>,
}

impl IntoResponse for SessionCreated {
    fn into_response(self) -> axum::response::Response {
        let redirect_to = self.redirect_to.as_ref().and_then(|r| r.path_and_query());

        match redirect_to {
            Some(redirect_to) => (self.session, Redirect::to(redirect_to.as_str())).into_response(),
            None => (self.session, StatusCode::CREATED).into_response(),
        }
    }
}

impl From<oxidrive_accounts::session::CreateSessionError> for ApiError {
    fn from(err: oxidrive_accounts::session::CreateSessionError) -> Self {
        match err {
            oxidrive_accounts::session::CreateSessionError::LoadCredentialsFailed(err) => {
                ApiError::new(err)
            }
            oxidrive_accounts::session::CreateSessionError::InvalidCredentials(_) => {
                ApiError::unauthenticated()
            }
            oxidrive_accounts::session::CreateSessionError::SaveSessionFailed(err) => {
                ApiError::new(err)
            }
        }
    }
}

#[derive(Clone)]
pub struct CreateSessionError {
    redirect_to: Option<Url>,
    error: ApiError,
}

impl CreateSessionError {
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

impl IntoResponse for CreateSessionError {
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
