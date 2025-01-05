use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use oxidrive_files::{
    file::{AllOwnedByError, ByIdError, ByNameError, FileId},
    File, Files,
};
use oxidrive_tags::{ForFileError, IndexTagError, ParseError, Tags};
use serde::{Deserialize, Serialize};

use crate::{
    paginate::{Page, PageParams},
    session::CurrentUser,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list))
        .route("/{file_id}/tags", get(tags).post(add_tags))
}

#[axum::debug_handler(state = AppState)]
async fn list(
    State(files): State<Files>,
    CurrentUser(account): CurrentUser,
    PageParams(params): PageParams,
) -> Result<Json<Page<FileData>>, ListError> {
    let files = files.metadata().all_owned_by(account.id, params).await?;
    Ok(Json(files.map(FileData::from).into()))
}

#[axum::debug_handler(state = AppState)]
async fn tags(
    State(tags): State<Tags>,
    State(files): State<Files>,
    CurrentUser(account): CurrentUser,
    Path(file_id): Path<FileId>,
) -> Result<Json<Vec<Tag>>, TagsError> {
    let Some(file) = files.metadata().by_id(account.id, file_id).await? else {
        return Err(TagsError::FileNotFound);
    };

    let tags = tags
        .for_file(file.id)
        .await?
        .into_iter()
        .map(Tag::from)
        .collect();
    Ok(Json(tags))
}

#[axum::debug_handler(state = AppState)]
async fn add_tags(
    State(tags): State<Tags>,
    State(files): State<Files>,
    CurrentUser(account): CurrentUser,
    Path(file_id): Path<FileId>,
    Json(body): Json<AddTags>,
) -> Result<TagsAdded, AddTagsError> {
    let Some(file) = files.metadata().by_id(account.id, file_id).await? else {
        return Err(AddTagsError::FileNotFound);
    };

    let tt: Result<Vec<_>, _> = body
        .tags
        .into_iter()
        .map(oxidrive_tags::Tag::parse)
        .collect();

    tags.index(&file, tt?).await?;

    Ok(TagsAdded)
}

#[derive(Debug, Serialize)]
struct FileData {
    id: String,
    name: String,
    content_type: String,
}

impl From<File> for FileData {
    fn from(file: File) -> Self {
        Self {
            id: file.id.to_string(),
            name: file.name,
            content_type: file.content_type,
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum ListError {
    #[error(transparent)]
    LoadError(#[from] AllOwnedByError),
}

impl IntoResponse for ListError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::LoadError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")).into_response()
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Tag {
    key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
}

impl From<oxidrive_tags::Tag> for Tag {
    fn from(tag: oxidrive_tags::Tag) -> Self {
        Self {
            key: tag.key,
            value: tag.value,
        }
    }
}

impl From<Tag> for oxidrive_tags::Tag {
    fn from(tag: Tag) -> Self {
        Self {
            key: tag.key,
            value: tag.value,
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum TagsError {
    #[error(transparent)]
    LoadFileError(#[from] ByIdError),
    #[error(transparent)]
    LoadTagsError(#[from] ForFileError),
    #[error("file not found")]
    FileNotFound,
}

impl IntoResponse for TagsError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::LoadFileError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")).into_response()
            }
            Self::LoadTagsError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")).into_response()
            }
            Self::FileNotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

#[derive(Deserialize)]
struct AddTags {
    tags: Vec<String>,
}

struct TagsAdded;

impl IntoResponse for TagsAdded {
    fn into_response(self) -> axum::response::Response {
        StatusCode::CREATED.into_response()
    }
}

#[derive(Debug, thiserror::Error)]
enum AddTagsError {
    #[error(transparent)]
    LoadFileError(#[from] ByIdError),
    #[error(transparent)]
    AddTagsError(#[from] IndexTagError),
    #[error(transparent)]
    InvalidTag(#[from] ParseError),
    #[error("file not found")]
    FileNotFound,
}

impl IntoResponse for AddTagsError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::LoadFileError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")).into_response()
            }
            Self::AddTagsError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")).into_response()
            }
            Self::FileNotFound => StatusCode::NOT_FOUND.into_response(),
            Self::InvalidTag(err) => (StatusCode::BAD_REQUEST, format!("{err}")).into_response(),
        }
    }
}
