use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use oxidrive_domain::make_error_wrapper;
use tokio::sync::RwLock;

use super::{Account, AccountId};

mod pg;
mod sqlite;

pub use pg::PgAccounts;
pub use sqlite::SqliteAccounts;

make_error_wrapper!(CountError);
make_error_wrapper!(ByIdError);
make_error_wrapper!(ByUsernameError);
make_error_wrapper!(SaveAccountError);

#[async_trait]
pub trait Accounts: Send + Sync + 'static {
    async fn count(&self) -> Result<usize, CountError>;
    async fn by_id(&self, id: AccountId) -> Result<Option<Account>, ByIdError>;
    async fn by_username(&self, username: &str) -> Result<Option<Account>, ByUsernameError>;
    async fn save(&self, account: Account) -> Result<Account, SaveAccountError>;
}

#[derive(Clone, Default)]
pub struct InMemoryAccounts {
    inner: Arc<RwLock<HashMap<AccountId, Account>>>,
}

impl<const N: usize> From<[Account; N]> for InMemoryAccounts {
    fn from(accounts: [Account; N]) -> Self {
        let accounts = HashMap::from_iter(accounts.into_iter().map(|a| (a.id, a)));
        Self {
            inner: Arc::new(RwLock::new(accounts)),
        }
    }
}

#[async_trait]
impl Accounts for InMemoryAccounts {
    async fn count(&self) -> Result<usize, CountError> {
        let inner = self.inner.read().await;
        Ok(inner.len())
    }

    async fn by_id(&self, id: AccountId) -> Result<Option<Account>, ByIdError> {
        let inner = self.inner.read().await;
        Ok(inner.get(&id).cloned())
    }

    async fn by_username(&self, username: &str) -> Result<Option<Account>, ByUsernameError> {
        let inner = self.inner.read().await;
        Ok(inner.values().find(|a| a.username == username).cloned())
    }

    async fn save(&self, account: Account) -> Result<Account, SaveAccountError> {
        let mut inner = self.inner.write().await;
        inner.insert(account.id, account.clone());
        Ok(account)
    }
}

#[cfg(test)]
mod tests;
