use axum::{
    Router,
    http::{HeaderMap, StatusCode, Uri, header},
    response::{Html, IntoResponse, Response},
};
use oxidrive_ui::{AssetFile, Assets};

use crate::{Config, state::AppState};

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

            let content_type = asset.content_type().map(str::to_string).unwrap_or_else(|| {
                mime_guess::from_path(path)
                    .first_or_octet_stream()
                    .essence_str()
                    .into()
            });

            headers.insert(
                header::CONTENT_TYPE,
                header::HeaderValue::from_str(&content_type).unwrap(),
            );

            if let Some(last_modified) = asset.last_modified() {
                headers.insert(
                    header::LAST_MODIFIED,
                    header::HeaderValue::from_str(&last_modified.to_string()).unwrap(),
                );
            }

            (headers, asset.into_data()).into_response()
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
        Some(asset) => Html(asset.into_data()).into_response(),
        None => not_found(),
    }
}

fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}
