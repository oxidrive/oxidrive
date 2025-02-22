use std::{collections::HashSet, marker::PhantomData};

use axum::{
    body::Bytes,
    extract::{Path, Query, State},
    http::{
        HeaderMap, HeaderValue, StatusCode,
        header::{self, IF_NONE_MATCH},
    },
    response::{IntoResponse, Response},
};
use axum_extra::response::FileStream;
use futures::Stream;
use oxidrive_accounts::auth::AccountEntity;
use oxidrive_authorization::Authorizer;
use oxidrive_files::{DownloadError, File, Files, auth::FileEntity, file::ByNameError};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    api::error::{ApiError, ApiResult},
    session::CurrentUser,
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
#[axum::debug_handler(state = crate::state::AppState)]
pub async fn handler(
    State(authorizer): State<Authorizer>,
    State(files): State<Files>,
    CurrentUser(account): CurrentUser,
    Path(file_name): Path<String>,
    Query(DownloadQuery { force }): Query<DownloadQuery>,
    headers: HeaderMap,
) -> ApiResult<Response> {
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

    let body = match etag_matches(&file, headers) {
        Some(_) => None,
        None => {
            let Some(body) = files.download(&file).await? else {
                return Err(ApiError::not_found());
            };
            Some(body)
        }
    };

    Ok(DownloadResponse { file, force, body }.into_response())
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
    body: Option<S>,
}

impl<S, E> IntoResponse for DownloadResponse<S>
where
    S: Stream<Item = Result<Bytes, E>> + Send + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    fn into_response(self) -> Response {
        let content_disposition = if self.force {
            attachment(&self.file)
        } else {
            content_disposition(&self.file)
        };

        let mut headers: HeaderMap = HeaderMap::from_iter([
            (header::CONTENT_DISPOSITION, content_disposition),
            (header::CONTENT_TYPE, header_value(&self.file.content_type)),
            (header::CACHE_CONTROL, HeaderValue::from_static("private")),
        ]);

        if let Some(hash) = self.file.hash() {
            headers.insert(header::ETAG, header_value(hash.to_string()));
        }

        let Some(body) = self.body else {
            return (StatusCode::NOT_MODIFIED, headers).into_response();
        };

        let body = FileStream::new(body).file_name(self.file.name);

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

fn content_disposition(file: &File) -> HeaderValue {
    if can_be_inlined(file) {
        return HeaderValue::from_static("inline");
    }

    attachment(file)
}

fn attachment(file: &File) -> HeaderValue {
    header_value(format!("attachment; filename=\"{}\"", file.name))
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

fn header_value(value: impl AsRef<str>) -> HeaderValue {
    HeaderValue::from_str(value.as_ref()).unwrap()
}

fn etag_matches(file: &File, headers: HeaderMap) -> Option<()> {
    let matching_etags = headers.get(IF_NONE_MATCH).and_then(|h| h.to_str().ok())?;

    // * matches anything
    if matching_etags == "*" {
        return Some(());
    }

    let hash = file.hash()?.to_string();

    let matching_etags = matching_etags
        .split(',')
        .map(|s| s.trim())
        .collect::<HashSet<_>>();

    if matching_etags.contains(hash.as_str()) {
        return Some(());
    }

    None
}
