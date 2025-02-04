use axum::{
    http::{header, HeaderMap, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    Router,
};
use oxidrive_ui::Assets;

use crate::{state::AppState, Config};

static INDEX_HTML: &str = "index.html";

pub fn routes(cfg: &Config) -> Router<AppState> {
    Router::new().fallback(static_handler).layer(cfg.csrf())
}

#[axum::debug_handler]
async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html();
    }

    match Assets::get(path) {
        Some(asset) => {
            let mut headers = HeaderMap::with_capacity(3);

            headers.insert(
                header::CONTENT_TYPE,
                header::HeaderValue::from_str(&asset.content_type).unwrap(),
            );
            headers.insert(
                header::CONTENT_LENGTH,
                header::HeaderValue::from_str(&asset.content_length.to_string()).unwrap(),
            );

            if let Some(last_modified) = asset.last_modified {
                headers.insert(
                    header::LAST_MODIFIED,
                    header::HeaderValue::from_str(&last_modified.to_string()).unwrap(),
                );
            }

            (headers, asset.bytes).into_response()
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
        Some(asset) => Html(asset.bytes).into_response(),
        None => not_found(),
    }
}

fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}
