use std::path::Path;

use figment::{
    providers::{Env, Format, Toml, Yaml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config<Telemetry, Web> {
    #[serde(default)]
    pub telemetry: Telemetry,
    #[serde(default)]
    pub server: Web,
}

impl<T, W> Config<T, W>
where
    T: for<'a> Deserialize<'a> + Default,
    W: for<'a> Deserialize<'a> + Default,
{
    pub fn load_from(path: impl AsRef<Path>) -> Config<T, W> {
        let path = path.as_ref();

        Figment::new()
            .merge(Env::prefixed("OXIDRIVE_").split('_'))
            .merge(Env::raw().only(&["HOST", "PORT"]).map(|k| {
                // make them equivalent to OXIDRIVE_SERVER_HOST and OXIDRIVER_SERVER_PORT
                format!("server.{k}").into()
            }))
            .merge(Toml::file(path.with_extension("toml")))
            .merge(Yaml::file(path.with_extension("yaml")))
            .merge(Yaml::file(path.with_extension("yml")))
            .extract()
            .unwrap()
    }
}
