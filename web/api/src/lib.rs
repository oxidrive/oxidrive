use std::{fmt::Display, sync::Arc};

use instance::InstanceService;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    RequestBuilder, Url,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sessions::SessionService;

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

    pub fn instance(&self) -> InstanceService {
        InstanceService::new(self.client.clone())
    }

    pub fn sessions(&self) -> SessionService {
        SessionService::new(self.client.clone())
    }
}

#[derive(Clone)]
struct Client {
    base_url: Arc<Url>,
    inner: reqwest::Client,
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
        Self { base_url, inner }
    }

    fn get(&self, path: impl AsRef<str>) -> RequestBuilder {
        let url = self.base_url.join(path.as_ref()).unwrap();
        self.inner.get(url)
    }

    fn post(&self, path: impl AsRef<str>) -> RequestBuilder {
        let url = self.base_url.join(path.as_ref()).unwrap();
        self.inner.post(url)
    }
}

#[derive(Clone, Debug, thiserror::Error, PartialEq, Deserialize, Serialize)]
#[error("error ({error}): {message}")]
pub struct ErrorResponse<E: Display> {
    pub error: E,
    pub message: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError<E: Display> {
    #[error(transparent)]
    Api(#[from] ErrorResponse<E>),
    #[error(transparent)]
    Network(#[from] reqwest::Error),
}

pub(crate) trait ApiErrorFromResponse<E: Display> {
    async fn check_error_response(self) -> ApiResult<Self, E>
    where
        Self: Sized;
}

impl<E: Display + DeserializeOwned> ApiErrorFromResponse<E> for reqwest::Response {
    async fn check_error_response(self) -> ApiResult<Self, E>
    where
        Self: Sized,
    {
        let status = self.status();
        if !status.is_client_error() && !status.is_server_error() {
            return Ok(self);
        }

        let error: ErrorResponse<E> = self.json().await?;
        Err(ApiError::Api(error))
    }
}

pub type ApiResult<T, E> = Result<T, ApiError<E>>;

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
