use crate::{
    account::{Account, Credentials, Password},
    Auth,
};

const DEFAULT_ADMIN_USERNAME: &str = "admin";

impl Auth {
    pub async fn upsert_initial_admin(
        &self,
        force: bool,
    ) -> app::eyre::Result<Option<InitialAdmin>> {
        if self.accounts.count().await? != 0 {
            if force {
                tracing::warn!(
                    "force recreating default admin account. This will overwrite its password"
                );
            } else {
                tracing::debug!("accounts found in database, skipping default admin creation");
                return Ok(None);
            }
        }

        let account = Account::create(DEFAULT_ADMIN_USERNAME);

        let account = if force {
            self.accounts
                .by_username(DEFAULT_ADMIN_USERNAME)
                .await?
                .unwrap_or(account)
        } else {
            account
        };

        let password = generate_pwd();
        let mut credentials = Credentials::new(account.id);
        credentials.add(Password::hash(&password)?)?;

        self.accounts.save(account).await?;
        self.credentials.save(credentials).await?;

        Ok(Some(InitialAdmin {
            username: DEFAULT_ADMIN_USERNAME.to_string(),
            password,
        }))
    }
}

pub struct InitialAdmin {
    pub username: String,
    pub password: String,
}

fn generate_pwd() -> String {
    use rand::distributions::{Alphanumeric, DistString};

    Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
}
