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
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::{self, ApiDoc},
    auth, files,
    state::AppState,
    ui, Config,
};

pub fn openapi_router(cfg: &Config) -> OpenApiRouter<AppState> {
    let mut router = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api", api::routes(cfg))
        .nest("/files", files::routes(cfg));

    api::finalize(router.get_openapi_mut());

    router
}

pub fn routes(cfg: &Config) -> Router<AppState> {
    let (router, api) = openapi_router(cfg).split_for_parts();

    router
        .route("/", get(root))
        .nest("/auth", auth::routes(cfg))
        .nest("/ui", ui::routes(cfg))
        .merge(SwaggerUi::new("/api/docs").url("/api/openapi.json", api))
}

#[axum::debug_handler]
async fn root(headers: HeaderMap) -> impl IntoResponse {
    let header = headers
        .get("accept")
        .and_then(|h| h.to_str().ok())
        .unwrap_or(APPLICATION_JSON.essence_str());

    let accept = AcceptHeader::from_str(header).unwrap_or_default();

    if accept.contains(|accept| {
        matches!(
            (accept.type_(), accept.subtype()),
            (_, HTML) | (TEXT, STAR) | (STAR, STAR)
        )
    }) {
        return Redirect::permanent("/ui").into_response();
    }

    tracing::warn!("unexpected accept header: {header}");
    StatusCode::NOT_IMPLEMENTED.into_response()
}

#[derive(Debug, Default)]
struct AcceptHeader {
    types: Vec<Mime>,
}

impl AcceptHeader {
    fn contains<F>(&self, predicate: F) -> bool
    where
        F: FnMut(&Mime) -> bool,
    {
        self.types.iter().any(predicate)
    }
}

impl FromStr for AcceptHeader {
    type Err = mime_guess::mime::FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s.chars().filter(|c| *c == ',').count();
        let mut types = Vec::with_capacity(items);

        for typ in s.split(',') {
            let mime = Mime::from_str(typ.trim())?;
            types.push(mime);
        }

        Ok(Self { types })
    }
}
