use std::{fmt::Display, sync::Arc};

use files::FileService;
use instance::InstanceService;
use models::error::Error;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    RequestBuilder, Url,
};
use sessions::SessionService;

pub mod models;

pub mod files;
pub mod instance;
pub mod sessions;

static USER_AGENT: &str = concat!("oxidrive-web", "/", env!("CARGO_PKG_VERSION"));

#[derive(Clone)]
pub struct Oxidrive {
    client: Client,
}

impl Oxidrive {
    pub fn new(base_url: impl AsRef<str>) -> Self {
        Self {
            client: Client::new(base_url),
        }
    }

    pub fn files(&self) -> FileService {
        FileService::new(self.client.clone())
    }

    pub fn instance(&self) -> InstanceService {
        InstanceService::new(self.client.clone())
    }

    pub fn sessions(&self) -> SessionService {
        SessionService::new(self.client.clone())
    }

    pub fn set_token(&mut self, token: impl ToString) {
        self.client.set_token(token)
    }

    pub fn remove_token(&mut self) {
        self.client.remove_token()
    }
}

#[derive(Clone)]
struct Client {
    base_url: Arc<Url>,
    inner: reqwest::Client,
    token: Option<String>,
}

impl Client {
    fn new(base_url: impl AsRef<str>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            HeaderValue::from_static("application/json"),
        );

        let inner = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .default_headers(headers)
            .build()
            .unwrap();

        let base_url = Arc::new(Url::parse(base_url.as_ref()).unwrap());
        Self {
            base_url,
            inner,
            token: None,
        }
    }

    pub fn set_token(&mut self, token: impl ToString) {
        self.token = Some(token.to_string());
    }

    pub fn remove_token(&mut self) {
        self.token = None;
    }

    fn get(&self, path: impl AsRef<str>) -> RequestBuilder {
        let url = self.base_url.join(path.as_ref()).unwrap();
        let req = self.inner.get(url);

        match self.token.as_ref() {
            Some(token) => req.bearer_auth(token),
            None => req,
        }
    }

    fn post(&self, path: impl AsRef<str>) -> RequestBuilder {
        let url = self.base_url.join(path.as_ref()).unwrap();
        let req = self.inner.post(url);

        match self.token.as_ref() {
            Some(token) => req.bearer_auth(token),
            None => req,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error(transparent)]
    Api(#[from] Error),
    #[error(transparent)]
    Network(#[from] reqwest::Error),
}

pub(crate) trait ApiErrorFromResponse {
    async fn check_error_response(self) -> ApiResult<Self>
    where
        Self: Sized;
}

impl ApiErrorFromResponse for reqwest::Response {
    async fn check_error_response(self) -> ApiResult<Self>
    where
        Self: Sized,
    {
        let status = self.status();
        if !status.is_client_error() && !status.is_server_error() {
            return Ok(self);
        }

        let error: Error = self.json().await?;
        Err(ApiError::Api(error))
    }
}

pub type ApiResult<T> = Result<T, ApiError>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error, self.message)
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
pub(crate) mod tests {
    use serde::Serialize;

    pub fn json<T: Serialize>(body: &T) -> Vec<u8> {
        serde_json::to_vec(body).unwrap()
    }

    pub fn json_val<T: Serialize>(body: &T) -> serde_json::Value {
        serde_json::to_value(body).unwrap()
    }
}
