use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;
use oxidrive_config::Config;
use oxidrive_telemetry as telemetry;
use oxidrive_web::WebModule;
use oxidrive_web::{self as web, Server};

type FullConfig = Config<telemetry::Config, web::Config>;

#[derive(Debug, Parser)]
struct Args {
    #[arg(
        long,
        short,
        env = "OXIDRIVE_CONFIG_FILE_PATH",
        default_value = "./oxidrive.yaml"
    )]
    config_file: PathBuf,
}

#[tokio::main]
async fn main() {
    telemetry::install_panic_logger();

    let args = Args::parse();
    let cfg: FullConfig = Config::load_from(&args.config_file);

    telemetry::init(&cfg.telemetry);

    bootstrap(cfg).run(run).await;
}

async fn run(c: Arc<app::di::Container>) -> eyre::Result<()> {
    let server = c.get::<Server>();

    tracing::info!(
        "oxidrive server listening on http://{}",
        server.local_address()
    );
    server.run().await?;
    Ok(())
}

fn bootstrap(cfg: FullConfig) -> app::App {
    app::app!().add(cfg.server).mount(WebModule)
}
