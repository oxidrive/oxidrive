use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use tower_http::catch_panic::CatchPanicLayer;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::{
        self,
        error::{handle_panic, ApiError},
        ApiDoc,
    },
    auth, files,
    headers::Accept,
    session::CurrentUser,
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

pub fn routes(cfg: &Config, state: AppState) -> Router {
    let (router, api) = openapi_router(cfg).split_for_parts();

    router
        .route("/", get(root))
        .nest("/auth", auth::routes(cfg))
        .nest("/ui", ui::routes(cfg))
        .merge(swagger_ui(state.clone(), api))
        .layer(CatchPanicLayer::custom(handle_panic))
        .with_state(state)
}

#[axum::debug_handler]
async fn root(accept: Option<Accept>) -> impl IntoResponse {
    let accept = accept.unwrap_or_else(Accept::json);

    if accept.accepts_html() {
        return Redirect::permanent("/ui").into_response();
    }

    tracing::warn!("unexpected accept header: {accept}");
    StatusCode::NOT_IMPLEMENTED.into_response()
}

fn swagger_ui(state: AppState, api: utoipa::openapi::OpenApi) -> Router<AppState> {
    let router: Router<AppState> = SwaggerUi::new("/api/docs")
        .url("/api/openapi.json", api)
        .into();

    router.layer(axum::middleware::from_fn_with_state(
        state,
        require_authentication,
    ))
}

#[axum::debug_middleware(state = crate::state::AppState)]
async fn require_authentication(
    current_user: Option<CurrentUser>,
    accept: Option<Accept>,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    if current_user.is_some() {
        return next.run(request).await;
    }

    let accept = accept.unwrap_or_else(Accept::json);
    if accept.accepts_html() {
        return Redirect::temporary(&format!("/ui/login?redirect_to={}", request.uri()))
            .into_response();
    }

    ApiError::unauthenticated().into_response()
}
