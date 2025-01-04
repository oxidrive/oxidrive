use async_trait::async_trait;
use uuid::Uuid;

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
        let count: i64 = sqlx::query_scalar("select count(id) as count from accounts")
            .fetch_one(&self.pool)
            .await
            .map_err(CountError::wrap)?;

        count.try_into().map_err(CountError::wrap)
    }

    async fn by_id(&self, id: AccountId) -> Result<Option<Account>, ByIdError> {
        let account: Option<PgAccount> = sqlx::query_as(
            r#"
select
  a.id,
  a.username
from accounts a
where
  id = $1
"#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(ByIdError::wrap)?;

        Ok(account.map(Into::into))
    }

    async fn by_username(&self, username: &str) -> Result<Option<Account>, ByUsernameError> {
        let account: Option<PgAccount> = sqlx::query_as(
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
        .bind(account.id.as_uuid())
        .bind(&account.username)
        .execute(&self.pool)
        .await
        .map_err(SaveAccountError::wrap)?;

        Ok(account)
    }
}

#[derive(sqlx::FromRow)]
struct PgAccount {
    id: Uuid,
    username: String,
}

impl From<PgAccount> for Account {
    fn from(account: PgAccount) -> Self {
        Account {
            id: account.id.into(),
            username: account.username,
        }
    }
}

#[cfg(test)]
mod tests {}
