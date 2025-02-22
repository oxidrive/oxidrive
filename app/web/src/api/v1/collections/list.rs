use axum::{Json, extract::State};

use oxidrive_files::collection::{AllOwnedByError, Collections};

use crate::{
    api::{
        error::{ApiError, ApiResult},
        v1::collections::CollectionData,
    },
    paginate::{Page, PageParams},
    session::CurrentUser,
};

#[utoipa::path(
    get,
    path = "/",
    operation_id = "list",
    responses((status = OK, body = Page<CollectionData>)),
    tag = "collections",
)]
#[axum::debug_handler(state = crate::state::AppState)]
pub async fn handler(
    State(collections): State<Collections>,
    CurrentUser(account): CurrentUser,
    PageParams(params): PageParams,
) -> ApiResult<Json<Page<CollectionData>>> {
    let collections = collections.all_owned_by(account.id, params).await?;

    Ok(Json(collections.map(CollectionData::from).into()))
}

impl From<AllOwnedByError> for ApiError {
    fn from(err: AllOwnedByError) -> Self {
        Self::new(err)
    }
}
