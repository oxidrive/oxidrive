use async_trait::async_trait;

use crate::account::Account;

use super::*;

pub struct PgAccounts {
    pool: sqlx::PgPool,
}

impl PgAccounts {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Accounts for PgAccounts {
    async fn count(&self) -> Result<usize, CountError> {
        Ok(0)
    }

    async fn by_id(&self, id: AccountId) -> Result<Option<Account>, ByIdError> {
        Ok(None)
    }

    async fn by_username(&self, username: &str) -> Result<Option<Account>, ByUsernameError> {
        Ok(None)
    }

    async fn save(&self, account: Account) -> Result<Account, SaveError> {
        todo!()
    }
}
