use axum::{
    extract::{Path, State},
    Json,
};
use oxidrive_accounts::auth::AccountEntity;
use oxidrive_authorization::Authorizer;
use oxidrive_files::{
    auth::FileEntity,
    file::{ByIdError, FileId},
    Files,
};

use crate::{
    api::{
        error::{ApiError, ApiResult},
        v1::files::FileData,
    },
    session::CurrentUser,
};

#[utoipa::path(
    get,
    path = "/{file_id}",
    operation_id = "get",
    params(("file_id" = String, Path, format = "uuid")),
    responses((status = 200, body = FileData)),
    tag = "files",
)]
#[axum::debug_handler(state = crate::state::AppState)]
pub async fn handler(
    State(authorizer): State<Authorizer>,
    State(files): State<Files>,
    CurrentUser(account): CurrentUser,
    Path(file_id): Path<FileId>,
) -> ApiResult<Json<FileData>> {
    let Some(file) = files.metadata().by_id(file_id).await? else {
        return Err(ApiError::not_found());
    };

    authorizer
        .authorize(
            &AccountEntity::from(&account),
            "get",
            &FileEntity::from(&file),
        )
        .into_err::<ApiError>()?;

    Ok(Json(file.into()))
}

impl From<ByIdError> for ApiError {
    fn from(err: ByIdError) -> Self {
        Self::new(err)
    }
}
