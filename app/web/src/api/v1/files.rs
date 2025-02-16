use delete::FileDeleted;
use oxidrive_files::File;
use serde::{Deserialize, Serialize};
use update::FileUpdated;
use utoipa::{OpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::state::AppState;

mod delete;
mod get;
mod list;
mod update;

#[derive(OpenApi)]
#[openapi(components(responses(FileUpdated, FileDeleted)))]
pub struct FilesApi;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list::handler))
        .routes(routes!(get::handler, update::handler, delete::handler))
}

#[derive(Debug, Serialize, ToSchema)]
struct FileData {
    id: String,
    name: String,
    content_type: String,
    size: usize,
    tags: Vec<Tag>,
}

impl From<File> for FileData {
    fn from(file: File) -> Self {
        let mut tags: Vec<Tag> = file.tags.into_values().map(Tag::from).collect();
        tags.sort();
        Self {
            id: file.id.to_string(),
            name: file.name,
            content_type: file.content_type,
            size: file.size,
            tags,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq, Eq, PartialOrd, Ord)]
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
