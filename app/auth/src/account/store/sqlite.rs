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
        let count: i64 = sqlx::query_scalar("select count(id) as count from accounts")
            .fetch_one(&self.pool)
            .await
            .map_err(CountError::wrap)?;

        count.try_into().map_err(CountError::wrap)
    }

    async fn by_id(&self, id: AccountId) -> Result<Option<Account>, ByIdError> {
        let account: Option<SqliteAccount> = sqlx::query_as(
            r#"
select
  a.id,
  a.username
from accounts a
where
  id = $1
"#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(ByIdError::wrap)?;

        Ok(account.map(Into::into))
    }

    async fn by_username(&self, username: &str) -> Result<Option<Account>, ByUsernameError> {
        let account: Option<SqliteAccount> = sqlx::query_as(
            r#"
select
  a.id,
  a.username
from accounts a
where
  username = $1
"#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(ByUsernameError::wrap)?;

        Ok(account.map(Into::into))
    }

    async fn save(&self, account: Account) -> Result<Account, SaveAccountError> {
        sqlx::query(
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
        )
        .bind(account.id.to_string())
        .bind(&account.username)
        .execute(&self.pool)
        .await
        .map_err(SaveAccountError::wrap)?;

        Ok(account)
    }
}

#[derive(sqlx::FromRow)]
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
