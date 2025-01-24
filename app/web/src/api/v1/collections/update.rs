use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use oxidrive_files::collection::{self, CollectionId, Collections, UpdateCollectionError};
use serde::Deserialize;
use utoipa::{ToResponse, ToSchema};

use crate::{
    api::error::{ApiError, ApiResult},
    state::AppState,
};

use super::CollectionData;

#[utoipa::path(
    patch,
    path = "/{collection_id}",
    operation_id = "update",
    responses((status = OK, response = CollectionUpdated)),
    tag = "collections",
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler(
    State(collections): State<Collections>,
    Path(id): Path<CollectionId>,
    Json(UpdateCollection { name, filter }): Json<UpdateCollection>,
) -> ApiResult<CollectionUpdated> {
    let collection = collections
        .update(id, collection::UpdateCollection { name, filter })
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
            UpdateCollectionError::NotFound(_) => Self::new(err).status(StatusCode::NOT_FOUND),
            UpdateCollectionError::LoadFailed(err) => Self::new(err),
        }
    }
}
