use axum::{
    body::Bytes,
    extract::{multipart::MultipartError, DefaultBodyLimit, Multipart, Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_extra::response::FileStream;
use futures::Stream;
use oxidrive_files::{File, Files, UploadMetadata};

use crate::{session::CurrentUser, state::AppState};

const TEN_GIGABYTES: usize = 10 * 1073741824;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(files)
                .post(upload)
                .route_layer(DefaultBodyLimit::max(TEN_GIGABYTES)),
        )
        .route("/{file_name}", get(download))
}

#[axum::debug_handler]
async fn files() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

#[axum::debug_handler(state = AppState)]
async fn download(
    State(files): State<Files>,
    CurrentUser(account): CurrentUser,
    Path(file_name): Path<String>,
) -> Result<impl IntoResponse, DownloadError> {
    let Some((file, body)) = files.download(&account, &file_name).await? else {
        return Err(DownloadError::FileNotFound);
    };

    Ok(DownloadResponse { file, body })
}

#[axum::debug_handler(state = AppState)]
async fn upload(
    State(files): State<Files>,
    CurrentUser(account): CurrentUser,
    mut body: Multipart,
) -> Result<UploadCompleted, UploadError> {
    let Some(field) = body.next_field().await? else {
        return Err(UploadError::InvalidBody);
    };

    match field.name() {
        Some("file") => {
            let file_name = field
                .file_name()
                .ok_or_else(|| UploadError::MissingFileName)?
                .to_string();
            files
                .upload(
                    UploadMetadata {
                        file_name,
                        owner_id: account.id,
                    },
                    field,
                )
                .await?;
            Ok(UploadCompleted {})
        }
        Some(name) => Err(UploadError::UnexpectedField(name.into())),
        None => Err(UploadError::FieldMissingName),
    }
}

pub struct DownloadResponse<S> {
    file: File,
    body: S,
}

impl<S, E> IntoResponse for DownloadResponse<S>
where
    S: Stream<Item = Result<Bytes, E>> + Send + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    fn into_response(self) -> axum::response::Response {
        let headers = [(header::CONTENT_TYPE, self.file.content_type)];

        let body = FileStream::new(self.body).file_name(self.file.name);

        (headers, body).into_response()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error("not found")]
    FileNotFound,
    #[error(transparent)]
    DownloadFailed(#[from] oxidrive_files::DownloadError),
}

impl IntoResponse for DownloadError {
    fn into_response(self) -> axum::response::Response {
        match self {
            DownloadError::DownloadFailed(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")).into_response()
            }
            DownloadError::FileNotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

pub struct UploadCompleted {}

impl IntoResponse for UploadCompleted {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED).into_response()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UploadError {
    #[error("multipart form field does not have a name")]
    FieldMissingName,
    #[error("filename not provided")]
    MissingFileName,
    #[error("field 'file' is required")]
    InvalidBody,
    #[error("unexpected field '{0}'")]
    UnexpectedField(String),
    #[error(transparent)]
    Multipart(#[from] MultipartError),
    #[error(transparent)]
    UploadFailed(#[from] oxidrive_files::UploadError),
}

impl IntoResponse for UploadError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Multipart(err) => err.into_response(),
            Self::FieldMissingName => {
                (StatusCode::BAD_REQUEST, format!("{self:?}")).into_response()
            }
            Self::UnexpectedField(err) => {
                (StatusCode::BAD_REQUEST, format!("{err:?}")).into_response()
            }
            Self::MissingFileName | Self::InvalidBody => {
                (StatusCode::BAD_REQUEST, format!("{self:?}")).into_response()
            }
            Self::UploadFailed(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")).into_response()
            }
        }
    }
}
