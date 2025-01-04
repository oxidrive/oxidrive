use axum::{
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    Router,
};
use rust_embed::Embed;

use crate::state::AppState;

static INDEX_HTML: &str = "index.html";

pub fn routes() -> Router<AppState> {
    Router::new().fallback(static_handler)
}

#[derive(Embed)]
#[folder = "../../web/build"]
struct Assets;

#[axum::debug_handler]
async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html();
    }

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if path.contains('.') {
                return not_found();
            }

            index_html()
        }
    }
}

fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found(),
    }
}

fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}
