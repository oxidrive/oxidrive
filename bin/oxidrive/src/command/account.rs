use clap::Subcommand;
use oxidrive_accounts::{Auth, CreateAccountError};

#[derive(Debug, clap::Args)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

impl Args {
    pub async fn run(&self, c: &app::di::Container) -> app::eyre::Result<()> {
        match &self.command {
            Command::Create(args) => create(c, args).await,
        }
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    Create(Create),
}

#[derive(Debug, clap::Args)]
struct Create {
    username: String,
    #[arg(short, long, env = "OXIDRIVE_CREATE_ACCOUNT_PASSWORD")]
    password: String,
    #[arg(long, env = "OXIDRIVE_CREATE_ACCOUNT_IF_NOT_EXISTS")]
    /// Don't error out if the account already exists
    if_not_exists: bool,
}

async fn create(
    c: &app::di::Container,
    Create {
        username,
        password,
        if_not_exists,
    }: &Create,
) -> app::eyre::Result<()> {
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
