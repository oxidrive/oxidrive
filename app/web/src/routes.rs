use std::str::FromStr;

use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use mime_guess::{
    mime::{APPLICATION_JSON, HTML, STAR, TEXT},
    Mime,
};

use crate::{api, state::AppState, ui};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/api", api::routes().with_state(state))
        .nest("/ui", ui::routes())
}

#[axum::debug_handler]
async fn root(headers: HeaderMap) -> impl IntoResponse {
    let accept = headers
        .get("accept")
        .and_then(|h| h.to_str().ok())
        .and_then(|accept| Mime::from_str(accept).ok())
        .unwrap_or(APPLICATION_JSON);

    match (accept.type_(), accept.subtype()) {
        (_, HTML) | (TEXT, STAR) | (STAR, STAR) => Redirect::permanent("/ui").into_response(),
        _ => StatusCode::NOT_IMPLEMENTED.into_response(),
    }
}
