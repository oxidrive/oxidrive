use async_trait::async_trait;
use oxidrive_domain::make_error_wrapper;

use super::{Account, AccountId};

mod pg;
mod sqlite;

pub use pg::PgAccounts;
pub use sqlite::SqliteAccounts;

make_error_wrapper!(CountError);
make_error_wrapper!(ByIdError);
make_error_wrapper!(ByUsernameError);
make_error_wrapper!(SaveError);

#[async_trait]
pub trait Accounts: Send + Sync + 'static {
    async fn count(&self) -> Result<usize, CountError>;
    async fn by_id(&self, id: AccountId) -> Result<Option<Account>, ByIdError>;
    async fn by_username(&self, username: &str) -> Result<Option<Account>, ByUsernameError>;
    async fn save(&self, account: Account) -> Result<Account, SaveError>;
}
