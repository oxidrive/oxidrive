use async_trait::async_trait;
use oxidrive_domain::make_error_wrapper;

use crate::account::AccountId;

use super::Credentials;

mod sqlite;

pub use sqlite::SqliteAccountCredentials;

make_error_wrapper!(ForAccountError);
make_error_wrapper!(SaveError);

#[async_trait]
pub trait AccountCredentials: Send + Sync + 'static {
    async fn for_account(&self, account_id: AccountId) -> Result<Credentials, ForAccountError>;
    async fn save(&self, credentials: Credentials) -> Result<Credentials, SaveError>;
}
