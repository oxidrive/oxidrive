use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use oxidrive_accounts::auth::AccountEntity;
use oxidrive_authorization::Authorizer;
use oxidrive_files::{FileId, Files, auth::FileEntity, file::DeleteFileError};
use utoipa::ToResponse;

use crate::{
    api::error::{ApiError, ApiResult, ApiResultExt},
    session::CurrentUser,
};

use super::FileData;

#[utoipa::path(
    delete,
    path = "/{file_id}",
    operation_id = "delete",
    params(("file_id" = String, format = "uuid")),
    responses((status = OK, response = FileDeleted)),
    tag = "files",
)]
#[axum::debug_handler(state = crate::state::AppState)]
pub async fn handler(
    State(authorizer): State<Authorizer>,
    State(files): State<Files>,
    CurrentUser(account): CurrentUser,
    Path(file_id): Path<FileId>,
) -> ApiResult<FileDeleted> {
    let Some(file) = files.metadata().by_id(file_id).await? else {
        return Err(ApiError::not_found());
    };

    authorizer
        .authorize(
            &AccountEntity::from(&account),
            "delete",
            &FileEntity::from(&file),
        )
        .into_err::<ApiError>()
        .hide_403_as_404()?;

    files.delete(&file).await?;

    Ok(FileDeleted(file.into()))
}

#[derive(ToResponse)]
pub struct FileDeleted(FileData);

impl IntoResponse for FileDeleted {
    fn into_response(self) -> axum::response::Response {
        Json(self.0).into_response()
    }
}

impl From<DeleteFileError> for ApiError {
    fn from(err: DeleteFileError) -> Self {
        Self::new(err)
    }
}
