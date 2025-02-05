use std::str::FromStr;

use axum::{extract::State, http::StatusCode, Json};
use axum_extra::extract::Query;
use oxidrive_files::{
    file::{AllOwnedByError, AllOwnedByInError},
    Files, SearchError,
};
use serde::{Deserialize, Deserializer};
use utoipa::IntoParams;
use uuid::Uuid;

use crate::{
    api::{
        error::{ApiError, ApiResult},
        v1::files::FileData,
    },
    paginate::{Page, PageParams},
    session::CurrentUser,
};

#[utoipa::path(
    get,
    path = "/",
    operation_id = "list",
    params(ListQuery),
    responses((status = OK, body = Page<FileData>)),
    tag = "files",
)]
#[axum::debug_handler(state = crate::state::AppState)]
pub async fn handler(
    State(files): State<Files>,
    CurrentUser(account): CurrentUser,
    Query(ListQuery { search, ids }): Query<ListQuery>,
    PageParams(params): PageParams,
) -> ApiResult<Json<Page<FileData>>> {
    if search.is_some() && !ids.is_empty() {
        return Err(
            ApiError::new("query parameters `search` and `ids` are mutually exclusive")
                .status(StatusCode::BAD_REQUEST)
                .error("INVALID_QUERY_PARAMS"),
        );
    }

    let files = match search {
        Some(filter) => files.search(account.id, filter, params).await?,
        None => {
            if ids.is_empty() {
                files.metadata().all_owned_by(account.id, params).await?
            } else {
                let ids = ids.into_iter().map(Into::into).collect::<Vec<_>>();
                files
                    .metadata()
                    .all_owned_by_in(account.id, &ids, params)
                    .await?
            }
        }
    };
    Ok(Json(files.map(FileData::from).into()))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListQuery {
    /// The OxiQL filter to search files for.
    /// Mutually exclusive with `ids`
    #[serde(
        alias = "q",
        alias = "query",
        default,
        deserialize_with = "empty_string_as_none"
    )]
    search: Option<String>,

    /// The list of File IDs to load. Non-existent IDs will be ignored.
    /// Mutually exclusive with `search`
    #[serde(rename = "id", default)]
    ids: Vec<Uuid>,
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

impl From<AllOwnedByInError> for ApiError {
    fn from(err: AllOwnedByInError) -> Self {
        Self::new(err)
    }
}

// from https://github.com/tokio-rs/axum/blob/main/examples/query-params-with-empty-strings/src/main.rs
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s)
            .map_err(serde::de::Error::custom)
            .map(Some),
    }
}
