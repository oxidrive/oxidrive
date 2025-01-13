use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use oxidrive_files::{file::AllOwnedByError, Files, SearchError};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::{
    api::{
        error::{ApiError, ApiResult},
        v1::files::FileData,
    },
    paginate::{Page, PageParams},
    session::CurrentUser,
    state::AppState,
};

#[utoipa::path(
    get,
    path = "/",
    operation_id = "list",
    params(ListQuery),
    responses((status = OK, body = Page<FileData>)),
    tag = "files",
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler(
    State(files): State<Files>,
    CurrentUser(account): CurrentUser,
    Query(ListQuery { search }): Query<ListQuery>,
    PageParams(params): PageParams,
) -> ApiResult<Json<Page<FileData>>> {
    let files = match search {
        Some(query) => files.search(account.id, query, params).await?,
        None => files.metadata().all_owned_by(account.id, params).await?,
    };
    Ok(Json(files.map(FileData::from).into()))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListQuery {
    #[serde(alias = "q", alias = "query")]
    search: Option<String>,
}

impl From<SearchError> for ApiError {
    fn from(err: SearchError) -> Self {
        match err {
            SearchError::QueryParse(err) => Self::new(err)
                .status(StatusCode::BAD_REQUEST)
                .error("INVALID_QUERY"),
            SearchError::SearchFailed(err) => Self::new(err),
        }
    }
}

impl From<AllOwnedByError> for ApiError {
    fn from(err: AllOwnedByError) -> Self {
        Self::new(err)
    }
}
