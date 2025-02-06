use std::path::PathBuf;

use serde::Deserialize;

use super::FileStorage;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub root_folder_path: PathBuf,
}

impl From<Config> for opendal::services::FsConfig {
    fn from(cfg: Config) -> Self {
        let mut svc = Self::default();
        svc.root = Some(cfg.root_folder_path.as_os_str().to_string_lossy().into());

        svc
    }
}

impl FileStorage {
    pub fn file_system(cfg: Config) -> Self {
        Self::new(opendal::services::FsConfig::from(cfg))
    }
}
