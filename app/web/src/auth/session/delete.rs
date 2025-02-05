use axum::http::header::REFERER;
use axum::http::StatusCode;
use axum::response::{Redirect, Response};
use axum::{http::HeaderMap, response::IntoResponse};

use crate::session::Session;

#[axum::debug_handler(state = crate::state::AppState)]
pub async fn handler(headers: HeaderMap, session: Session) -> Response {
    // TODO: replace with this as soon as a new docs.rs/headers release is cut
    // let referer = headers
    //     .typed_get::<Referer>()
    //     .map(|referer| referer.to_string());

    let referer = headers
        .get(REFERER)
        .and_then(|referer| referer.to_str().ok());

    let session = session.clear();

    if let Some(referer) = referer {
        return (session, Redirect::to(referer)).into_response();
    }

    (session, StatusCode::OK).into_response()
}
