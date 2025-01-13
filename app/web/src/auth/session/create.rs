use axum::{
    extract::{Query, State},
    http::{self, HeaderMap, StatusCode, Uri},
    response::{IntoResponse, Redirect},
    Form,
};
use axum_extra::extract::cookie::SignedCookieJar;
use headers::{HeaderMapExt, Referer};
use oxidrive_auth::{login::AuthenticationFailed, Auth};
use serde::Deserialize;
use url::Url;

use crate::{api::error::ApiError, session::Session, state::AppState};

#[axum::debug_handler(state = AppState)]
pub async fn handler(
    State(auth): State<Auth>,
    jar: SignedCookieJar,
    Query(query): Query<CreateQuery>,
    headers: HeaderMap,
    Form(creds): Form<SessionCredentials>,
) -> Result<SessionCreated, CreateSessionError> {
    let referer = headers
        .typed_get::<Referer>()
        .and_then(|referer| Url::parse(&referer.to_string()).ok());
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

    let account = match creds {
        SessionCredentials::Password { username, password } => auth
            .login()
            .password(&username, &password)
            .await
            .map_err(ApiError::from)
            .map_err(|err| error.wrap(err))?,
    };

    let session = Session::create(account, jar);

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

#[derive(Debug)]
pub struct SessionCreated {
    session: Session,
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

impl From<AuthenticationFailed> for ApiError {
    fn from(err: AuthenticationFailed) -> Self {
        match err {
            AuthenticationFailed::ByUsernameError(err) => Self::new(err),
            AuthenticationFailed::CredentialsError(err) => Self::new(err),
            AuthenticationFailed::Unauthorized => Self::unauthorized(),
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
