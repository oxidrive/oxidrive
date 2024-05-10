use std::sync::Arc;

use instance::InstanceService;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    RequestBuilder, Url,
};

pub mod instance;

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

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error(transparent)]
    Network(#[from] reqwest::Error),
}

pub type ApiResult<T> = Result<T, ApiError>;
