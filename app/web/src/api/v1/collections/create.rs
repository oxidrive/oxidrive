use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use oxidrive_files::collection::{self, Collections, CreateCollectionError};
use serde::Deserialize;
use utoipa::{ToResponse, ToSchema};

use crate::{
    api::error::{ApiError, ApiResult},
    session::CurrentUser,
};

use super::CollectionData;

#[utoipa::path(
    post,
    path = "/",
    operation_id = "create",
    responses((status = CREATED, response = CollectionCreated)),
    tag = "collections",
)]
#[axum::debug_handler(state = crate::state::AppState)]
pub async fn handler(
    State(collections): State<Collections>,
    CurrentUser(account): CurrentUser,
    Json(CreateCollection { name, filter }): Json<CreateCollection>,
) -> ApiResult<CollectionCreated> {
    let collection = collections
        .create(account.id, collection::CreateCollection { name, filter })
        .await?;

    Ok(CollectionCreated(collection.into()))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCollection {
    name: String,
    filter: String,
}

#[derive(Debug, ToResponse)]
#[response(content_type = "application/json")]
pub struct CollectionCreated(CollectionData);

impl IntoResponse for CollectionCreated {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, Json(self.0)).into_response()
    }
}

impl From<CreateCollectionError> for ApiError {
    fn from(err: CreateCollectionError) -> Self {
        match err {
            CreateCollectionError::FilterParse(err) => Self::new(err)
                .status(StatusCode::BAD_REQUEST)
                .error("INVALID_QUERY"),
            CreateCollectionError::SaveFailed(err) => Self::new(err),
        }
    }
}
