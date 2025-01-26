use create::CollectionCreated;
use oxidrive_files::collection::Collection;
use serde::Serialize;
use update::CollectionUpdated;
use utoipa::{OpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::state::AppState;

mod create;
mod get;
mod list;
mod update;

#[derive(OpenApi)]
#[openapi(components(responses(CollectionCreated, CollectionUpdated)))]
pub struct CollectionsApi;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create::handler, list::handler))
        .routes(routes!(get::handler, update::handler))
}

#[derive(Debug, Serialize, ToSchema)]
struct CollectionData {
    id: Uuid,
    name: String,
    files: Vec<Uuid>,
}

impl From<Collection> for CollectionData {
    fn from(collection: Collection) -> Self {
        Self {
            id: collection.id.as_uuid(),
            files: collection.files().map(|id| id.as_uuid()).collect(),
            name: collection.name,
        }
    }
}
