use std::sync::Arc;

use account::{
    Account, AccountCredentials, Accounts, ByUsernameError, Credentials, ForAccountError,
    HashError, InvalidCredentials, Password, PgAccountCredentials, PgAccounts, SaveAccountError,
    SaveCredentialsError, SqliteAccountCredentials, SqliteAccounts, VerifyCreds,
};
use oxidrive_database::Database;
use pat::{PersonalAccessTokens, PersonalAccessTokensModule};
use session::{Sessions, SessionsModule, jobs::JobsModule};

pub mod account;
pub mod auth;
pub mod pat;
pub mod session;

mod setup;

#[derive(Clone)]
pub struct AccountService {
    accounts: Arc<dyn Accounts>,
    credentials: Arc<dyn AccountCredentials>,
    sessions: Sessions,
    pats: PersonalAccessTokens,
}

impl AccountService {
    pub fn new(
        accounts: Arc<dyn Accounts>,
        credentials: Arc<dyn AccountCredentials>,
        sessions: Sessions,
        pats: PersonalAccessTokens,
    ) -> Self {
        Self {
            accounts,
            credentials,
            sessions,
            pats,
        }
    }

    pub fn accounts(&self) -> &dyn Accounts {
        self.accounts.as_ref()
    }

    pub fn sessions(&self) -> &Sessions {
        &self.sessions
    }

    pub fn personal_access_tokens(&self) -> &PersonalAccessTokens {
        &self.pats
    }

    pub async fn create_account(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Account, CreateAccountError> {
        if self.accounts.by_username(username).await?.is_some() {
            return Err(CreateAccountError::AlreadyExists);
        };

        let account = Account::create(username);
        let account = self.accounts.save(account).await?;

        let mut credentials = Credentials::new(account.id);
        credentials.add(Password::hash(password)?).unwrap();

        self.credentials.save(credentials).await?;

        Ok(account)
    }

    pub async fn verify_password(
        &self,
        account: &Account,
        password: impl Into<String>,
    ) -> Result<(), VerifyPasswordError> {
        let credentials = self.credentials.for_account(account.id).await?;
        credentials.verify(VerifyCreds::Password(password.into()))?;
        Ok(())
    }

    pub async fn change_password(
        &self,
        account: &Account,
        password: impl AsRef<str>,
    ) -> Result<(), ChangePasswordError> {
        let mut credentials = self.credentials.for_account(account.id).await?;
        credentials.replace(Password::hash(password.as_ref())?);

        self.credentials.save(credentials).await?;

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CreateAccountError {
    #[error(transparent)]
    Load(#[from] ByUsernameError),
    #[error("account already exists")]
    AlreadyExists,
    #[error(transparent)]
    SaveAccount(#[from] SaveAccountError),
    #[error(transparent)]
    InvalidPassword(#[from] HashError),
    #[error(transparent)]
    SaveCredentials(#[from] SaveCredentialsError),
}

#[derive(Debug, thiserror::Error)]
pub enum VerifyPasswordError {
    #[error(transparent)]
    LoadCredentials(#[from] ForAccountError),
    #[error(transparent)]
    InvalidPassword(#[from] InvalidCredentials),
}

#[derive(Debug, thiserror::Error)]
pub enum ChangePasswordError {
    #[error(transparent)]
    LoadCredentials(#[from] ForAccountError),
    #[error(transparent)]
    InvalidPassword(#[from] HashError),
    #[error(transparent)]
    SaveCredentials(#[from] SaveCredentialsError),
}

#[derive(Copy, Clone)]
pub struct AccountsModule;

impl app::Module for AccountsModule {
    fn mount(self: Box<Self>, c: &mut app::di::Context) {
        c.bind(accounts);
        c.bind(credentials);
        c.mount(SessionsModule);
        c.mount(PersonalAccessTokensModule);
        c.bind(AccountService::new);
        c.mount(JobsModule);
    }
}

fn accounts(database: Database) -> Arc<dyn Accounts> {
    match database {
        Database::Sqlite(pool) => Arc::new(SqliteAccounts::new(pool)),
        Database::Pg(pool) => Arc::new(PgAccounts::new(pool)),
    }
}

fn credentials(database: Database) -> Arc<dyn AccountCredentials> {
    match database {
        Database::Sqlite(pool) => Arc::new(SqliteAccountCredentials::new(pool)),
        Database::Pg(pool) => Arc::new(PgAccountCredentials::new(pool)),
    }
}

#[app::async_trait]
impl app::Hooks for AccountsModule {
    async fn after_start(
        &mut self,
        ctx: app::context::Context,
        c: &app::di::Container,
    ) -> app::eyre::Result<()> {
        JobsModule.after_start(ctx, c).await?;

        let auth = c.get::<AccountService>();

        if let Some(admin) = auth.upsert_initial_admin(false).await? {
            tracing::warn!(
                username = admin.username,
                password = admin.password,
                "no accounts found, generated default admin account"
            );
        }

        Ok(())
    }

    async fn before_start(
        &mut self,
        ctx: app::context::Context,
        c: &app::di::Container,
    ) -> app::eyre::Result<()> {
        JobsModule.before_start(ctx, c).await?;
        Ok(())
    }

    async fn on_shutdown(
        &mut self,
        ctx: app::context::Context,
        c: &app::di::Container,
    ) -> app::eyre::Result<()> {
        JobsModule.on_shutdown(ctx, c).await?;
        Ok(())
    }
}
