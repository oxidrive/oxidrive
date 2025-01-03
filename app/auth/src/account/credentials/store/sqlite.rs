use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, QueryBuilder};

use crate::account::{AccountId, Credentials, Creds, Password};

use super::*;

pub struct SqliteAccountCredentials {
    pool: sqlx::SqlitePool,
}

impl SqliteAccountCredentials {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccountCredentials for SqliteAccountCredentials {
    async fn for_account(&self, account_id: AccountId) -> Result<Credentials, ForAccountError> {
        let id = account_id.to_string();

        let creds = sqlx::query_as!(
            SqliteCredentials,
            r#"
select
  id,
  kind as "kind: SqliteCredentialsKind",
  data as "data: Json<SqliteCredentialsData>"
from account_credentials
where account_id = $1
"#,
            id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(ForAccountError::wrap)?;

        let mut credentials = Credentials::new(account_id);
        credentials.creds = creds.into_iter().map(SqliteCredentials::to_creds).collect();

        Ok(credentials)
    }

    async fn save(&self, credentials: Credentials) -> Result<Credentials, SaveError> {
        let mut qb =
            QueryBuilder::new("insert into account_credentials (id, account_id, kind, data)");

        qb.push_values(credentials.values(), |mut b, creds| {
            let (kind, data) = match creds {
                Creds::Password(password) => (
                    SqliteCredentialsKind::Password,
                    SqliteCredentialsData::Password(password.password_hash.clone()),
                ),
            };

            b.push_bind(creds.id())
                .push_bind(credentials.account_id.to_string())
                .push_bind(kind)
                .push_bind(Json(data));
        });

        qb.push(
            r#"
on conflict (id, account_id)
do update set
  kind = excluded.kind,
  data = excluded.data
"#,
        );

        qb.build()
            .execute(&self.pool)
            .await
            .map_err(SaveError::wrap)?;

        Ok(credentials)
    }
}

struct SqliteCredentials {
    id: String,
    kind: SqliteCredentialsKind,
    data: Json<SqliteCredentialsData>,
}

impl SqliteCredentials {
    fn to_creds(creds: SqliteCredentials) -> (String, Creds) {
        let id = creds.id;

        let creds = match (creds.kind, creds.data.0) {
            (SqliteCredentialsKind::Password, SqliteCredentialsData::Password(hash)) => {
                Password::from_hash(hash).into()
            }
        };

        (id, creds)
    }
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "kind")]
#[sqlx(rename_all = "lowercase")]
enum SqliteCredentialsKind {
    Password,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum SqliteCredentialsData {
    Password(String),
}

impl From<&Creds> for SqliteCredentialsData {
    fn from(creds: &Creds) -> Self {
        match creds {
            Creds::Password(password) => Self::Password(password.password_hash.clone()),
        }
    }
}

#[cfg(test)]
mod tests {}
