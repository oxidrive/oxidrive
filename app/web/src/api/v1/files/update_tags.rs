use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use oxidrive_files::{file::FileId, tag::ParseError, AddTagsError, Files};
use serde::Deserialize;
use utoipa::{ToResponse, ToSchema};

use crate::{
    api::error::{ApiError, ApiResult},
    session::CurrentUser,
    state::AppState,
};

use super::FileData;

#[utoipa::path(
    put,
    path = "/{file_id}/tags",
    operation_id = "tags::update",
    params(("file_id" = String, format = "uuid")),
    responses((status = OK, response = TagsUpdated)),
    tag = "files",
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler(
    State(files): State<Files>,
    CurrentUser(account): CurrentUser,
    Path(file_id): Path<FileId>,
    Json(body): Json<UpdateTags>,
) -> ApiResult<TagsUpdated> {
    let tags = body
        .tags
        .into_iter()
        .map(oxidrive_files::Tag::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let file = files.update_tags(account.id, file_id, tags).await?;

    Ok(TagsUpdated(file.into()))
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateTags {
    tags: Vec<String>,
}

#[derive(ToResponse)]
pub struct TagsUpdated(FileData);

impl IntoResponse for TagsUpdated {
    fn into_response(self) -> axum::response::Response {
        Json(self.0).into_response()
    }
}

impl From<ParseError> for ApiError {
    fn from(err: ParseError) -> Self {
        Self::new(err).status(StatusCode::BAD_REQUEST)
    }
}

impl From<AddTagsError> for ApiError {
    fn from(err: AddTagsError) -> Self {
        match err {
            AddTagsError::LoadFailed(err) => Self::new(err),
            AddTagsError::FileNotFound => Self::not_found(),
            AddTagsError::SaveFileFailed(err) => Self::new(err),
        }
    }
}
