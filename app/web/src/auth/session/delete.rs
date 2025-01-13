use axum::http::StatusCode;
use axum::response::{Redirect, Response};
use axum::{http::HeaderMap, response::IntoResponse};
use headers::HeaderMapExt;
use headers::Referer;

use crate::{session::Session, state::AppState};

#[axum::debug_handler(state = AppState)]
pub async fn handler(headers: HeaderMap, session: Session) -> Response {
    let referer = headers
        .typed_get::<Referer>()
        .map(|referer| referer.to_string());

    let session = session.clear();

    if let Some(referer) = referer {
        return (session, Redirect::to(&referer)).into_response();
    }

    (session, StatusCode::OK).into_response()
}
