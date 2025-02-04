use std::path::PathBuf;
use std::sync::Arc;

use clap::{Parser, Subcommand};
use oxidrive::{PoliciesModule, ServerModule};
use oxidrive_accounts::{Auth, AuthModule, CreateAccountError};
use oxidrive_config::Config;
use oxidrive_database::{self as database, Database, DatabaseModule};
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

#[derive(Debug, Subcommand)]
pub enum Command {
    Migrate,
    CreateDefaultAdmin,
    CreateAccount {
        username: String,
        #[arg(short, long, env = "OXIDRIVE_CREATE_ACCOUNT_PASSWORD")]
        password: String,
        #[arg(long, env = "OXIDRIVE_CREATE_ACCOUNT_IF_NOT_EXISTS")]
        /// Don't error out if the account already exists
        if_not_exists: bool,
    },
    Server,
    Worker,
}

impl Command {
    pub async fn run(&self, c: &app::di::Container) -> eyre::Result<()> {
        match self {
            Command::Migrate => oxidrive_database::migrate(c.get::<Database>()).await,
            Command::CreateDefaultAdmin => {
                let auth = c.get::<Auth>();
                let admin = auth.upsert_initial_admin(true).await?.unwrap();

                let out = serde_json::to_string_pretty(&serde_json::json!({
                    "username": admin.username,
                    "password": admin.password,
                }))?;

                println!();
                println!("{out}");
                println!();

                Ok(())
            }
            Command::CreateAccount {
                username,
                password,
                if_not_exists,
            } => {
                let auth = c.get::<Auth>();
                let result = auth.create_account(username, password).await;

                let account = if *if_not_exists {
                    match result {
                        Ok(account) => account,
                        Err(CreateAccountError::AlreadyExists) => {
                            tracing::info!(username, "account already exists");
                            return Ok(());
                        }
                        err => err?,
                    }
                } else {
                    result?
                };

                tracing::info!(id=%account.id, username, "account created");

                Ok(())
            }
            Command::Server => unreachable!(),
            Command::Worker => {
                todo!("workers")
            }
        }
    }
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
        .mount_and_hook(AuthModule)
        .mount_and_hook(FilesModule)
        .mount(WebModule)
}
