use axum::{
    extract::{multipart::MultipartError, Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use oxidrive_files::{Files, UploadMetadata};
use utoipa::{ToResponse, ToSchema};

use crate::{
    api::error::{ApiError, ApiResult},
    session::CurrentUser,
};

#[utoipa::path(
    post,
    path = "/",
    operation_id = "upload",
    request_body(content = inline(UploadForm), content_type = "multipart/form-data"),
    responses((status = CREATED, response = UploadCompleted)),
    tags = ["files", "content"],
)]
#[axum::debug_handler(state = crate::state::AppState)]
pub async fn handler(
    State(files): State<Files>,
    CurrentUser(account): CurrentUser,
    mut body: Multipart,
) -> ApiResult<UploadCompleted> {
    let Some(field) = body.next_field().await? else {
        return Err(UploadError::InvalidBody.into());
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
        }
        Some(name) => Err(UploadError::UnexpectedField(name.into()))?,
        None => Err(UploadError::FieldMissingName)?,
    };

    Ok(UploadCompleted)
}

#[derive(ToSchema)]
#[allow(unused)] // only used for utoipa schema generation
struct UploadForm {
    #[schema(format = Binary, content_media_type = "application/octet-stream")]
    file: String,
}

#[derive(ToResponse)]
pub struct UploadCompleted;

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

impl From<UploadError> for ApiError {
    fn from(err: UploadError) -> Self {
        match err {
            UploadError::FieldMissingName
            | UploadError::MissingFileName
            | UploadError::InvalidBody => Self::new(err).status(StatusCode::BAD_REQUEST),
            UploadError::UnexpectedField(err) => Self::new(err).status(StatusCode::BAD_REQUEST),
            UploadError::Multipart(err) => err.into(),
            UploadError::UploadFailed(err) => err.into(),
        }
    }
}

impl From<MultipartError> for ApiError {
    fn from(err: MultipartError) -> Self {
        let status = err.status();
        let message = err.body_text();

        Self::new(err).status(status).message(message)
    }
}

impl From<oxidrive_files::UploadError> for ApiError {
    fn from(err: oxidrive_files::UploadError) -> Self {
        Self::new(err)
    }
}
