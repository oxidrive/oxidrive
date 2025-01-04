use std::path::Path;

use figment::{
    providers::{Env, Format, Toml, Yaml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config<Telemetry, Web, Database, Storage> {
    #[serde(default)]
    pub telemetry: Telemetry,

    pub server: Web,

    pub database: Database,

    pub storage: Storage,
}

impl<T, W, D, S> Config<T, W, D, S>
where
    T: for<'a> Deserialize<'a> + Default,
    W: for<'a> Deserialize<'a>,
    D: for<'a> Deserialize<'a>,
    S: for<'a> Deserialize<'a>,
{
    pub fn load_from(path: impl AsRef<Path>) -> eyre::Result<Config<T, W, D, S>> {
        let path = path.as_ref();

        let cfg = Figment::new()
            .merge(Env::prefixed("OXIDRIVE_").split('_'))
            .merge(Env::raw().only(&["DATABASE_URL"]).split('_'))
            .merge(Env::raw().only(&["HOST", "PORT"]).map(|k| {
                // make them equivalent to OXIDRIVE_SERVER_HOST and OXIDRIVER_SERVER_PORT
                format!("server.{k}").into()
            }))
            .merge(Toml::file(path.with_extension("toml")))
            .merge(Yaml::file(path.with_extension("yaml")))
            .merge(Yaml::file(path.with_extension("yml")))
            .extract()?;

        Ok(cfg)
    }
}
