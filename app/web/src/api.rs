use axum::http::{header::CACHE_CONTROL, HeaderValue};
use error::{ApiError, ApiErrorBody};
use tower_http::set_header::SetResponseHeaderLayer;
use utoipa::{
    openapi::{
        path::Operation,
        security::{ApiKey, ApiKeyValue, SecurityScheme},
        PathItem,
    },
    Modify, OpenApi,
};
use utoipa_axum::router::OpenApiRouter;

use crate::{files, session::SESSION_COOKIE, state::AppState, Config};

pub mod error;

mod v1;

pub fn routes(cfg: &Config) -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/v1", v1::routes())
        .layer(cfg.cors())
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_static("no-cache"),
        ))
}

#[derive(OpenApi)]
#[openapi(
    info(title = "Oxidrive"),
    modifiers(&SecuritySchemes),
    components(schemas(ApiErrorBody), responses(ApiError)),
    nest(
        (path = "api/v1", api = v1::V1Api),
        (path = "files", api = files::FileContents),
    ),
)]
pub struct ApiDoc;

pub fn finalize(api: &mut utoipa::openapi::OpenApi) {
    PrependOperationPrefix.modify(api);
    AddErrorResponse.modify(api);
}

struct SecuritySchemes;

impl Modify for SecuritySchemes {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "session",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new(SESSION_COOKIE))),
            );
        }
    }
}

struct PrependOperationPrefix;

impl Modify for PrependOperationPrefix {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        for (path, item) in openapi.paths.paths.iter_mut() {
            let prefix = path
                .split('/')
                .skip(1)
                .take_while(|segment| !segment.starts_with('{'))
                .collect::<Vec<_>>()
                .join("::");

            for op in all_paths(item) {
                op.operation_id = op
                    .operation_id
                    .as_ref()
                    .map(|id| format!("{}::{id}", prefix));
            }
        }
    }
}

fn all_paths(path: &mut PathItem) -> impl Iterator<Item = &mut Operation> {
    [
        path.get.as_mut(),
        path.put.as_mut(),
        path.post.as_mut(),
        path.delete.as_mut(),
        path.options.as_mut(),
        path.head.as_mut(),
        path.patch.as_mut(),
        path.trace.as_mut(),
    ]
    .into_iter()
    .flatten()
}

struct AddErrorResponse;

impl Modify for AddErrorResponse {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        use utoipa::ToResponse;

        let (response, _) = ApiError::response();
        let response = utoipa::openapi::Ref::from_response_name(response);

        for op in openapi.paths.paths.values_mut().flat_map(all_paths) {
            op.responses
                .responses
                .insert("4XX".into(), response.clone().into());
            op.responses
                .responses
                .insert("5XX".into(), response.clone().into());
        }
    }
}
