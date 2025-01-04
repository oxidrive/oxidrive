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

use crate::{api, files, state::AppState, ui};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .nest("/api", api::routes())
        .nest("/files", files::routes())
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
