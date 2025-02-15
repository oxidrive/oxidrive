use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use oxidrive_accounts::auth::AccountEntity;
use oxidrive_authorization::Authorizer;
use oxidrive_files::{auth::FileEntity, file::FileId, tag::ParseError, AddTagsError, Files};
use serde::Deserialize;
use utoipa::{ToResponse, ToSchema};

use crate::{
    api::error::{ApiError, ApiResult},
    session::CurrentUser,
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
#[axum::debug_handler(state = crate::state::AppState)]
pub async fn handler(
    State(authorizer): State<Authorizer>,
    State(files): State<Files>,
    CurrentUser(account): CurrentUser,
    Path(file_id): Path<FileId>,
    Json(body): Json<UpdateTags>,
) -> ApiResult<TagsUpdated> {
    let Some(file) = files.metadata().by_id(file_id).await? else {
        return Err(ApiError::unauthorized());
    };

    authorizer
        .authorize(
            &AccountEntity::from(&account),
            "updateTags",
            &FileEntity::from(&file),
        )
        .into_err::<ApiError>()?;

    let tags = body
        .tags
        .into_iter()
        .map(oxidrive_files::Tag::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let file = files.update_tags(file, tags).await?;

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
            AddTagsError::SaveFileFailed(err) => Self::new(err),
        }
    }
}
