use clap::Subcommand;
use oxidrive_accounts::AccountService;
use oxidrive_database::Database;

mod account;

#[derive(Debug, Subcommand)]
pub enum Command {
    Migrate,
    CreateDefaultAdmin,
    Account(account::Args),
    Server,
    Worker,
}

impl Command {
    pub async fn run(
        &self,
        ctx: app::context::Context,
        c: &app::di::Container,
    ) -> eyre::Result<()> {
        match self {
            Command::Migrate => oxidrive_database::migrate(ctx, c.get::<Database>()).await,
            Command::CreateDefaultAdmin => {
                let accounts = c.get::<AccountService>();
                let admin = accounts.upsert_initial_admin(true).await?.unwrap();

                let out = serde_json::to_string_pretty(&serde_json::json!({
                    "username": admin.username,
                    "password": admin.password,
                }))?;

                println!();
                println!("{out}");
                println!();

                Ok(())
            }
            Command::Account(cmd) => cmd.run(ctx, c).await,
            Command::Server => unreachable!(),
            Command::Worker => {
                todo!("workers")
            }
        }
    }
}
