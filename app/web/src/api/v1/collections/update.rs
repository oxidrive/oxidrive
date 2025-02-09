use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use oxidrive_accounts::auth::AccountEntity;
use oxidrive_authorization::Authorizer;
use oxidrive_files::{
    auth::CollectionEntity,
    collection::{self, CollectionId, Collections, UpdateCollectionError},
};
use serde::Deserialize;
use utoipa::{ToResponse, ToSchema};

use crate::{
    api::error::{ApiError, ApiResult},
    session::CurrentUser,
};

use super::CollectionData;

#[utoipa::path(
    patch,
    path = "/{collection_id}",
    operation_id = "update",
    params(("collection_id" = String, Path, format = "uuid")),
    responses((status = OK, response = CollectionUpdated)),
    tag = "collections",
)]
#[axum::debug_handler(state = crate::state::AppState)]
pub async fn handler(
    State(authorizer): State<Authorizer>,
    State(collections): State<Collections>,
    CurrentUser(account): CurrentUser,
    Path(id): Path<CollectionId>,
    Json(UpdateCollection { name, filter }): Json<UpdateCollection>,
) -> ApiResult<CollectionUpdated> {
    let Some(collection) = collections.by_id(id).await? else {
        return Err(ApiError::not_found());
    };

    authorizer
        .authorize(
            &AccountEntity::from(&account),
            "get",
            &CollectionEntity::from(&collection),
        )
        .into_err::<ApiError>()?;

    let collection = collections
        .update(collection, collection::UpdateCollection { name, filter })
        .await?;

    Ok(CollectionUpdated(collection.into()))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateCollection {
    name: Option<String>,
    filter: Option<String>,
}

#[derive(Debug, ToResponse)]
#[response(content_type = "application/json")]
pub struct CollectionUpdated(CollectionData);

impl IntoResponse for CollectionUpdated {
    fn into_response(self) -> axum::response::Response {
        Json(self.0).into_response()
    }
}

impl From<UpdateCollectionError> for ApiError {
    fn from(err: UpdateCollectionError) -> Self {
        match err {
            UpdateCollectionError::FilterParse(err) => Self::new(err)
                .status(StatusCode::BAD_REQUEST)
                .error("INVALID_QUERY"),
            UpdateCollectionError::SaveFailed(err) => Self::new(err),
        }
    }
}
