use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use oxidrive_domain::make_error_wrapper;
use tokio::sync::RwLock;

use crate::account::AccountId;

use super::Credentials;

mod pg;
mod sqlite;

pub use pg::PgAccountCredentials;
pub use sqlite::SqliteAccountCredentials;

make_error_wrapper!(ForAccountError);
make_error_wrapper!(SaveCredentialsError);

#[async_trait]
pub trait AccountCredentials: Send + Sync + 'static {
    async fn for_account(&self, account_id: AccountId) -> Result<Credentials, ForAccountError>;
    async fn save(&self, credentials: Credentials) -> Result<Credentials, SaveCredentialsError>;
}

#[derive(Clone, Default)]
pub struct InMemoryCredentials {
    inner: Arc<RwLock<HashMap<AccountId, Credentials>>>,
}

#[async_trait]
impl AccountCredentials for InMemoryCredentials {
    async fn for_account(&self, account_id: AccountId) -> Result<Credentials, ForAccountError> {
        let inner = self.inner.read().await;

        Ok(inner
            .get(&account_id)
            .cloned()
            .unwrap_or_else(|| Credentials::new(account_id)))
    }

    async fn save(&self, credentials: Credentials) -> Result<Credentials, SaveCredentialsError> {
        if credentials.creds.is_empty() {
            return Ok(credentials);
        }

        let mut inner = self.inner.write().await;
        inner.insert(credentials.account_id, credentials.clone());
        Ok(credentials)
    }
}

#[cfg(test)]
mod tests;
