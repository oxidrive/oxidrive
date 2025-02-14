use std::{convert::Infallible, str::FromStr};

use clap::Subcommand;
use eyre::{Context, OptionExt};
use oxidrive_accounts::{AccountService, CreateAccountError};

#[derive(Debug, clap::Args)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

impl Args {
    pub async fn run(
        &self,
        _ctx: app::context::Context,
        c: &app::di::Container,
    ) -> app::eyre::Result<()> {
        match &self.command {
            Command::Create(args) => create(c, args).await,
            Command::ChangePassword(args) => change_password(c, args).await,
        }
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    Create(Create),
    ChangePassword(ChangePassword),
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
    let accounts = c.get::<AccountService>();
    let result = accounts.create_account(username, password).await;

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

#[derive(Debug, clap::Args)]
struct ChangePassword {
    username: String,

    /// The password to set.
    /// Pass `-` to read the password from stdin
    #[arg(short, long)]
    password: Password,
}

#[derive(Clone, Debug)]
enum Password {
    Text(String),
    Stdin,
}

impl FromStr for Password {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            return Ok(Self::Stdin);
        }

        Ok(Self::Text(s.into()))
    }
}

async fn change_password(
    c: &app::di::Container,
    ChangePassword { username, password }: &ChangePassword,
) -> app::eyre::Result<()> {
    let accounts = c.get::<AccountService>();

    let password = match password {
        Password::Text(password) => password,
        Password::Stdin => &std::io::stdin()
            .lines()
            .next()
            .ok_or_eyre("password is required")?
            .wrap_err("failed to read password from stdin")?,
    };

    let Some(account) = accounts.accounts().by_username(username).await? else {
        app::eyre::bail!("no account found by username {username}");
    };

    accounts.change_password(&account, password).await?;

    tracing::info!(id=%account.id, username, "account password changed");

    Ok(())
}
