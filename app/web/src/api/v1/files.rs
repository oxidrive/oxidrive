use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use oxidrive_files::{
    file::{AllOwnedByError, ByIdError, FileId},
    tag::ParseError,
    File, Files, SearchError,
};
use serde::{Deserialize, Serialize};

use crate::{
    paginate::{Page, PageParams},
    session::CurrentUser,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list))
        .route("/{file_id}", get(fetch))
        .route("/{file_id}/tags", post(add_tags))
}

#[axum::debug_handler(state = AppState)]
async fn list(
    State(files): State<Files>,
    CurrentUser(account): CurrentUser,
    Query(ListQuery { query }): Query<ListQuery>,
    PageParams(params): PageParams,
) -> Result<Json<Page<FileData>>, ListError> {
    let files = match query {
        Some(query) => files.search(account.id, query, params).await?,
        None => files.metadata().all_owned_by(account.id, params).await?,
    };
    Ok(Json(files.map(FileData::from).into()))
}

#[axum::debug_handler(state = AppState)]
async fn fetch(
    State(files): State<Files>,
    CurrentUser(account): CurrentUser,
    Path(file_id): Path<FileId>,
) -> Result<Json<FileData>, FetchError> {
    let Some(file) = files.metadata().by_id(account.id, file_id).await? else {
        return Err(FetchError::NotFound);
    };

    Ok(Json(file.into()))
}

#[axum::debug_handler(state = AppState)]
async fn add_tags(
    State(files): State<Files>,
    CurrentUser(account): CurrentUser,
    Path(file_id): Path<FileId>,
    Json(body): Json<AddTags>,
) -> Result<TagsAdded, AddTagsError> {
    let tags = body
        .tags
        .into_iter()
        .map(oxidrive_files::Tag::parse_public)
        .collect::<Result<Vec<_>, _>>()?;

    files.add_tags(account.id, file_id, tags).await?;

    Ok(TagsAdded)
}

#[derive(Debug, Deserialize)]
struct ListQuery {
    #[serde(alias = "q", alias = "search")]
    query: Option<String>,
}

#[derive(Debug, Serialize)]
struct FileData {
    id: String,
    name: String,
    content_type: String,
    size: usize,
    tags: Vec<Tag>,
}

impl From<File> for FileData {
    fn from(file: File) -> Self {
        Self {
            id: file.id.to_string(),
            name: file.name,
            content_type: file.content_type,
            size: file.size,
            tags: file.tags.into_iter().map(Tag::from).collect(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum ListError {
    #[error(transparent)]
    Search(#[from] SearchError),
    #[error(transparent)]
    Load(#[from] AllOwnedByError),
}

impl IntoResponse for ListError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Load(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")).into_response()
            }
            Self::Search(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")).into_response()
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum FetchError {
    #[error(transparent)]
    Load(#[from] ByIdError),
    #[error("file not found")]
    NotFound,
}

impl IntoResponse for FetchError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Load(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")).into_response()
            }
            Self::NotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Tag {
    key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
}

impl From<oxidrive_files::Tag> for Tag {
    fn from(tag: oxidrive_files::Tag) -> Self {
        Self {
            key: tag.key,
            value: tag.value,
        }
    }
}

impl From<Tag> for oxidrive_files::Tag {
    fn from(tag: Tag) -> Self {
        Self {
            key: tag.key,
            value: tag.value,
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
    AddTags(#[from] oxidrive_files::AddTagsError),
    #[error(transparent)]
    InvalidTag(#[from] ParseError),
}

impl IntoResponse for AddTagsError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::AddTags(oxidrive_files::AddTagsError::FileNotFound) => {
                StatusCode::NOT_FOUND.into_response()
            }
            Self::AddTags(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{err} ({err:?})"),
            )
                .into_response(),
            Self::InvalidTag(err) => (StatusCode::BAD_REQUEST, format!("{err}")).into_response(),
        }
    }
}
