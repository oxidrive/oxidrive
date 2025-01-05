use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use oxidrive_files::{file::AllOwnedByError, File, Files};
use serde::Serialize;

use crate::{
    paginate::{Page, PageParams},
    session::CurrentUser,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(list))
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
            ListError::LoadError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")).into_response()
            }
        }
    }
}
