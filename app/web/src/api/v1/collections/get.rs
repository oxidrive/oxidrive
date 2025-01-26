use axum::{
    extract::{Path, State},
    Json,
};
use oxidrive_accounts::auth::AccountEntity;
use oxidrive_authorization::Authorizer;
use oxidrive_files::{
    auth::CollectionEntity,
    collection::{ByIdError, CollectionId, Collections},
};

use crate::{
    api::error::{ApiError, ApiResult},
    session::CurrentUser,
    state::AppState,
};

use super::CollectionData;

#[utoipa::path(
    get,
    path = "/{collection_id}",
    operation_id = "get",
    params(("collection_id" = String, Path, format = "uuid")),
    responses((status = 200, body = CollectionData)),
    tag = "files",
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler(
    State(authorizer): State<Authorizer>,
    State(collections): State<Collections>,
    CurrentUser(account): CurrentUser,
    Path(collection_id): Path<CollectionId>,
) -> ApiResult<Json<CollectionData>> {
    let Some(collection) = collections.by_id(collection_id).await? else {
        return Err(ApiError::not_found());
    };

    authorizer
        .authorize(
            &AccountEntity::from(&account),
            "get",
            &CollectionEntity::from(&collection),
        )
        .into_err::<ApiError>()?;

    Ok(Json(collection.into()))
}

impl From<ByIdError> for ApiError {
    fn from(err: ByIdError) -> Self {
        Self::new(err)
    }
}
