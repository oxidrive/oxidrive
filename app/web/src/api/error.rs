use std::{
    any::Any,
    collections::HashMap,
    fmt::{Debug, Display},
};

use axum::{http::StatusCode, response::IntoResponse, Json};
use oxidrive_authorization::Authorized;
use serde::Serialize;
use utoipa::{openapi::Content, ToResponse, ToSchema};

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Clone, Debug)]
pub struct ApiError {
    status: StatusCode,
    error: Option<String>,
    message: String,
    details: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, ToSchema)]
#[schema(as = ApiError)]
pub struct ApiErrorBody {
    pub error: String,
    pub message: String,
    pub details: HashMap<String, serde_json::Value>,
}

impl ApiError {
    pub fn new<D: Display + Debug>(err: D) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: err.to_string(),
            error: None,
            details: HashMap::from_iter([("error".into(), format!("{err:?}").into())]),
        }
    }

    pub fn not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: "not found".into(),
            error: Some("NOT_FOUND".into()),
            details: HashMap::default(),
        }
    }

    pub fn unauthenticated() -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            message: "authentication required".into(),
            error: Some("UNAUTHENTICATED".into()),
            details: HashMap::default(),
        }
    }

    pub fn unauthorized() -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            message: "access denied".into(),
            error: Some("UNAUTHORIZED".into()),
            details: HashMap::default(),
        }
    }

    pub fn status(mut self, status: impl Into<StatusCode>) -> Self {
        self.status = status.into();
        self
    }

    #[allow(dead_code)]
    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    #[allow(dead_code)]
    pub fn details(mut self, details: impl Into<HashMap<String, serde_json::Value>>) -> Self {
        self.details = details.into();
        self
    }

    #[allow(dead_code)]
    pub fn detail(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.details.insert(key.into(), value.into());
        self
    }

    pub fn into_body(self) -> ApiErrorBody {
        ApiErrorBody {
            error: self
                .error
                .or_else(|| self.status.canonical_reason().map(Into::into))
                .unwrap_or_else(|| "UNKNOWN".into()),
            message: self.message,
            details: self.details,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status;
        let body = Json(self.into_body());

        (status, body).into_response()
    }
}

impl<'r> ToResponse<'r> for ApiError {
    fn response() -> (
        &'r str,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    ) {
        (
            "ApiError",
            utoipa::openapi::ResponseBuilder::new()
                .content(
                    "application/json",
                    Content::new(Some(utoipa::openapi::Ref::from_schema_name(
                        ApiErrorBody::name(),
                    ))),
                )
                .build()
                .into(),
        )
    }
}

impl From<Authorized> for ApiError {
    fn from(_: Authorized) -> Self {
        Self::unauthorized()
    }
}

pub fn handle_panic(err: Box<dyn Any + Send + 'static>) -> axum::response::Response {
    let details = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic message".to_string()
    };

    ApiError::new(details).error("UNEXPECTED").into_response()
}
