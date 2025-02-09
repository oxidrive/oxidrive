use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;
use oxidrive::{command::Command, PoliciesModule, ServerModule};
use oxidrive_accounts::AccountsModule;
use oxidrive_config::Config;
use oxidrive_database::{self as database, DatabaseModule};
use oxidrive_files::{self as files, FilesModule};
use oxidrive_telemetry as telemetry;
use oxidrive_web::{self as web, Server, WebModule};

type FullConfig = Config<telemetry::Config, web::Config, database::Config, files::Config>;

#[derive(Debug, Parser)]
struct Args {
    #[arg(
        long,
        short,
        env = "OXIDRIVE_CONFIG_FILE_PATH",
        default_value = "./oxidrive.yaml"
    )]
    config_file: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() {
    telemetry::install_panic_logger();

    let args = Args::parse();
    let cfg: FullConfig = match Config::load_from(&args.config_file) {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("oxidrive: invalid configuration: {err}");
            return;
        }
    };

    telemetry::init(&cfg.telemetry);

    let app = bootstrap(cfg);

    match args.command {
        Command::Server => app.run(run).await,
        cmd => {
            let name = app.name.clone();
            let c = app.init();
            if let Err(err) = cmd.run(&c).await {
                ::app::handle_error(&name, &err);
            }
        }
    }
}

async fn run(c: Arc<app::di::Container>) -> eyre::Result<()> {
    let server = c.get::<Server>();

    #[cfg(debug_assertions)]
    let _dev = oxidrive_web::start_dev_server();

    tracing::info!(
        "oxidrive server listening on http://{}",
        server.local_address()
    );
    server.run().await?;
    Ok(())
}

fn bootstrap(cfg: FullConfig) -> app::App {
    app::app!()
        .add(cfg.database)
        .add(cfg.server)
        .add(cfg.storage)
        .mount(PoliciesModule)
        .mount_and_hook(DatabaseModule)
        .mount_and_hook(ServerModule)
        .mount_and_hook(AccountsModule)
        .mount_and_hook(FilesModule)
        .mount(WebModule)
}
