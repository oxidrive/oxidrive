mod panic;

pub use panic::install_panic_logger;
use serde::Deserialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init(cfg: &Config) {
    let pretty_layer =
        is_format(cfg.format, LogFormat::Pretty).then(|| tracing_subscriber::fmt::layer().pretty());
    let text_layer = is_format(cfg.format, LogFormat::Text).then(tracing_subscriber::fmt::layer);
    let json_layer = is_format(cfg.format, LogFormat::Json)
        .then(|| tracing_subscriber::fmt::layer().json().flatten_event(true));

    let env_filter = EnvFilter::try_from_env("OXIDRIVE_LOG")
        .or_else(|_| EnvFilter::try_from_default_env())
        .or_else(|_| EnvFilter::try_new(&cfg.log))
        .unwrap();

    tracing_subscriber::registry()
        .with(env_filter)
        .with(pretty_layer)
        .with(text_layer)
        .with(json_layer)
        .init();
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_log")]
    log: String,

    #[serde(default = "default_format")]
    format: LogFormat,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log: default_log(),
            format: default_format(),
        }
    }
}

fn default_log() -> String {
    "info".into()
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum LogFormat {
    Pretty,
    Text,
    Json,
}

fn default_format() -> LogFormat {
    LogFormat::Text
}

fn is_format(actual: LogFormat, expected: LogFormat) -> bool {
    actual == expected
}
