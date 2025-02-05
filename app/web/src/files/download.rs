use std::marker::PhantomData;

use axum::{
    body::Bytes,
    extract::{Path, Query, State},
    http::header,
    response::IntoResponse,
};
use axum_extra::response::FileStream;
use futures::Stream;
use oxidrive_accounts::auth::AccountEntity;
use oxidrive_authorization::Authorizer;
use oxidrive_files::{auth::FileEntity, file::ByNameError, DownloadError, File, Files};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    api::error::{ApiError, ApiResult},
    session::CurrentUser,
    state::AppState,
};

#[utoipa::path(
    get,
    path = "/{file_name}",
    operation_id = "download",
    params(("file_name" = String, Path), ("force" = bool, Query)),
    responses((
        status = OK,
        description = "Raw content of the file. The actual content type varies based on the detected format",
        content_type = "application/octet-stream",
        body = inline(BinaryFile),
        example = "hello world",
    )),
    tags = ["files", "content"],
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler(
    State(authorizer): State<Authorizer>,
    State(files): State<Files>,
    CurrentUser(account): CurrentUser,
    Path(file_name): Path<String>,
    Query(DownloadQuery { force }): Query<DownloadQuery>,
) -> ApiResult<impl IntoResponse> {
    let Some(file) = files
        .metadata()
        .by_owner_and_name(account.id, &file_name)
        .await?
    else {
        return Err(ApiError::not_found());
    };

    authorizer
        .authorize(
            &AccountEntity::from(&account),
            "download",
            &FileEntity::from(&file),
        )
        .into_err::<ApiError>()?;

    let Some(body) = files.download(&file).await? else {
        return Err(ApiError::not_found());
    };

    Ok(DownloadResponse { file, force, body })
}

#[derive(ToSchema)]
#[allow(unused)] // only used for utoipa schema generation
#[schema(value_type = String, format = Binary)]
struct BinaryFile(PhantomData<Vec<u8>>);

#[derive(Debug, Deserialize)]
pub struct DownloadQuery {
    #[serde(default)]
    force: bool,
}

pub struct DownloadResponse<S> {
    file: File,
    force: bool,
    body: S,
}

impl<S, E> IntoResponse for DownloadResponse<S>
where
    S: Stream<Item = Result<Bytes, E>> + Send + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    fn into_response(self) -> axum::response::Response {
        let content_disposition = if self.force {
            attachment(&self.file)
        } else {
            content_disposition(&self.file)
        };

        let headers = [
            (header::CONTENT_DISPOSITION, content_disposition),
            (header::CONTENT_TYPE, self.file.content_type),
            (header::CACHE_CONTROL, "private".into()),
        ];

        let body = FileStream::new(self.body).file_name(self.file.name);

        (headers, body).into_response()
    }
}

impl From<ByNameError> for ApiError {
    fn from(err: ByNameError) -> Self {
        Self::new(err)
    }
}

impl From<DownloadError> for ApiError {
    fn from(err: DownloadError) -> Self {
        Self::new(err)
    }
}

fn content_disposition(file: &File) -> String {
    if can_be_inlined(file) {
        return "inline".into();
    }

    attachment(file)
}

fn attachment(file: &File) -> String {
    format!("attachment; filename=\"{}\"", file.name)
}

const INLINEABLE: &[&str] = &["application/pdf", "image/", "video/", "audio/"];

fn can_be_inlined(file: &File) -> bool {
    for prefix in INLINEABLE {
        if file.content_type.starts_with(prefix) {
            return true;
        }
    }

    false
}
