use accounts::AccountsApi;
use files::FilesApi;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

mod accounts;
mod files;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "accounts", api = AccountsApi, tags = ["accounts"]),
        (path = "files", api = FilesApi, tags = ["files"]),
    ),
)]
pub struct V1Api;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/accounts", accounts::routes())
        .nest("/files", files::routes())
}
