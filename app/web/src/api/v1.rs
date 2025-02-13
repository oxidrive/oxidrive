use accounts::AccountsApi;
use collections::CollectionsApi;
use files::FilesApi;
use pats::PatsApi;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

mod accounts;
mod collections;
mod files;
mod pats;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "accounts", api = AccountsApi, tags = ["accounts"]),
        (path = "collections", api = CollectionsApi, tags = ["collections"]),
        (path = "files", api = FilesApi, tags = ["files"]),
        (path = "pats", api = PatsApi, tags = ["pats"]),
    ),
)]
pub struct V1Api;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/accounts", accounts::routes())
        .nest("/collections", collections::routes())
        .nest("/files", files::routes())
        .nest("/pats", pats::routes())
}
