use axum::extract::DefaultBodyLimit;
use upload::UploadCompleted;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{state::AppState, Config};

mod download;
mod upload;

#[derive(OpenApi)]
#[openapi(components(responses(UploadCompleted)))]
pub struct FileContents;

pub fn routes(cfg: &Config) -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(upload::handler))
        .route_layer(DefaultBodyLimit::max(
            cfg.upload_body_limit.as_u64() as usize
        ))
        .routes(routes!(download::handler))
}
