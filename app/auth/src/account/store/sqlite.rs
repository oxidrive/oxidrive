use async_trait::async_trait;

use crate::account::Account;

use super::*;

pub struct SqliteAccounts {
    pool: sqlx::SqlitePool,
}

impl SqliteAccounts {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Accounts for SqliteAccounts {
    async fn count(&self) -> Result<usize, CountError> {
        let count = sqlx::query_scalar!("select count(id) as count from accounts")
            .fetch_one(&self.pool)
            .await
            .map_err(CountError::wrap)?;

        count.try_into().map_err(CountError::wrap)
    }

    async fn by_id(&self, id: AccountId) -> Result<Option<Account>, ByIdError> {
        let id = id.to_string();

        let account = sqlx::query_as!(
            SqliteAccount,
            r#"
select
  a.id,
  a.username
from accounts a
where
  id = $1
"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(ByIdError::wrap)?;

        Ok(account.map(Into::into))
    }

    async fn by_username(&self, username: &str) -> Result<Option<Account>, ByUsernameError> {
        let account = sqlx::query_as!(
            SqliteAccount,
            r#"
select
  a.id,
  a.username
from accounts a
where
  username = $1
"#,
            username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(ByUsernameError::wrap)?;

        Ok(account.map(Into::into))
    }

    async fn save(&self, account: Account) -> Result<Account, SaveError> {
        let id = account.id.to_string();

        sqlx::query!(
            r#"
insert into accounts (
  id,
  username
) values (
  $1,
  $2
)
on conflict(id)
do update set
  username = excluded.username
"#,
            id,
            account.username
        )
        .execute(&self.pool)
        .await
        .map_err(SaveError::wrap)?;

        Ok(account)
    }
}

struct SqliteAccount {
    id: String,
    username: String,
}

impl From<SqliteAccount> for Account {
    fn from(account: SqliteAccount) -> Self {
        Account {
            id: account.id.parse().unwrap(),
            username: account.username,
        }
    }
}

#[cfg(test)]
mod tests {}
