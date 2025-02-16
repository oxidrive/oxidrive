use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use oxidrive_accounts::auth::AccountEntity;
use oxidrive_authorization::Authorizer;
use oxidrive_files::{auth::FileEntity, file::FileId, tag::ParseError, Files, UpdateError};
use serde::Deserialize;
use utoipa::{ToResponse, ToSchema};

use crate::{
    api::error::{ApiError, ApiResult, ApiResultExt},
    session::CurrentUser,
};

use super::FileData;

#[utoipa::path(
    patch,
    path = "/{file_id}",
    operation_id = "update",
    params(("file_id" = String, format = "uuid")),
    responses((status = OK, response = FileUpdated)),
    tag = "files",
)]
#[axum::debug_handler(state = crate::state::AppState)]
pub async fn handler(
    State(authorizer): State<Authorizer>,
    State(files): State<Files>,
    CurrentUser(account): CurrentUser,
    Path(file_id): Path<FileId>,
    Json(body): Json<UpdateFile>,
) -> ApiResult<FileUpdated> {
    let Some(file) = files.metadata().by_id(file_id).await? else {
        return Err(ApiError::not_found());
    };

    authorizer
        .authorize(
            &AccountEntity::from(&account),
            "update",
            &FileEntity::from(&file),
        )
        .into_err::<ApiError>()
        .hide_403_as_404()?;

    let tags = match body.tags {
        Some(tags) => Some(
            tags.into_iter()
                .map(oxidrive_files::Tag::parse)
                .collect::<Result<Vec<_>, _>>()?,
        ),
        None => None,
    };

    let file = files
        .update(
            file,
            oxidrive_files::file::UpdateFile {
                name: body.name,
                tags,
            },
        )
        .await?;

    Ok(FileUpdated(file.into()))
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateFile {
    name: Option<String>,
    tags: Option<Vec<String>>,
}

#[derive(ToResponse)]
pub struct FileUpdated(FileData);

impl IntoResponse for FileUpdated {
    fn into_response(self) -> axum::response::Response {
        Json(self.0).into_response()
    }
}

impl From<ParseError> for ApiError {
    fn from(err: ParseError) -> Self {
        Self::new(err).status(StatusCode::BAD_REQUEST)
    }
}

impl From<UpdateError> for ApiError {
    fn from(err: UpdateError) -> Self {
        match err {
            UpdateError::SaveFileFailed(err) => Self::new(err),
        }
    }
}
