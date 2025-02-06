use std::fmt::Display;

use serde::Deserialize;

use super::FileStorage;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub bucket: String,
    pub prefix: Option<String>,
    pub endpoint: Option<String>,
    pub region: Option<String>,
    #[serde(default)]
    pub url_style: UrlStyle,
    pub storage_class: Option<StorageClass>,

    #[serde(flatten)]
    pub credentials: Option<StaticCredentials>,
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UrlStyle {
    Path,
    Vhost,
}

impl Default for UrlStyle {
    fn default() -> Self {
        Self::Path
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StorageClass {
    DeepArchive,
    Glacier,
    GlacierIr,
    IntelligentTiering,
    OnezoneIa,
    Outposts,
    ReducedRedundancy,
    Standard,
    StandardIa,
}

impl Display for StorageClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeepArchive => "DEEP_ARCHIVE",
            Self::Glacier => "GLACIER",
            Self::GlacierIr => "GLACIER_IR",
            Self::IntelligentTiering => "INTELLIGENT_TIERING",
            Self::OnezoneIa => "ONEZONE_IA",
            Self::Outposts => "OUTPOSTS",
            Self::ReducedRedundancy => "REDUCED_REDUNDANCY",
            Self::Standard => "STANDARD",
            Self::StandardIa => "STANDARD_IA",
        }
        .fmt(f)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct StaticCredentials {
    pub access_key: String,
    pub secret_key: String,
}

impl From<Config> for opendal::services::S3Config {
    fn from(cfg: Config) -> Self {
        let mut svc = Self::default();

        svc.bucket = cfg.bucket;
        svc.root = cfg.prefix;
        svc.endpoint = cfg.endpoint;
        svc.region = cfg.region;
        svc.enable_virtual_host_style = matches!(cfg.url_style, UrlStyle::Vhost);
        svc.default_storage_class = cfg.storage_class.map(|c| c.to_string());

        if let Some(StaticCredentials {
            access_key,
            secret_key,
        }) = cfg.credentials
        {
            svc.access_key_id = Some(access_key);
            svc.secret_access_key = Some(secret_key);
        }

        svc
    }
}

impl FileStorage {
    pub fn s3(cfg: Config) -> Self {
        Self::new(opendal::services::S3Config::from(cfg))
    }
}
