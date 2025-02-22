use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{QueryBuilder, types::Json};

use crate::account::{AccountId, Credentials, Creds, Password};

use super::*;

pub struct PgAccountCredentials {
    pool: sqlx::PgPool,
}

impl PgAccountCredentials {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccountCredentials for PgAccountCredentials {
    async fn for_account(&self, account_id: AccountId) -> Result<Credentials, ForAccountError> {
        let creds: Vec<PgCredentials> = sqlx::query_as(
            r#"
select
  id,
  kind,
  data
from account_credentials
where account_id = $1
"#,
        )
        .bind(account_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(ForAccountError::wrap)?;

        let mut credentials = Credentials::new(account_id);
        credentials.creds = creds.into_iter().map(PgCredentials::to_creds).collect();

        Ok(credentials)
    }

    async fn save(&self, credentials: Credentials) -> Result<Credentials, SaveCredentialsError> {
        if credentials.creds.is_empty() {
            return Ok(credentials);
        }

        let account_id = credentials.account_id.as_uuid();

        let mut qb =
            QueryBuilder::new("insert into account_credentials (id, account_id, kind, data)");

        qb.push_values(credentials.values(), |mut b, creds| {
            let (kind, data) = match creds {
                Creds::Password(password) => (
                    PgCredentialsKind::Password,
                    PgCredentialsData::Password(password.password_hash.clone()),
                ),
            };

            b.push_bind(creds.id())
                .push_bind(account_id)
                .push_bind(kind)
                .push_bind(Json(data));
        });

        qb.push(
            r#"
on conflict (account_id, id)
do update set
  kind = excluded.kind,
  data = excluded.data
"#,
        );

        qb.build()
            .execute(&self.pool)
            .await
            .map_err(SaveCredentialsError::wrap)?;

        Ok(credentials)
    }
}

#[derive(sqlx::FromRow)]
struct PgCredentials {
    id: String,
    kind: PgCredentialsKind,
    data: Json<PgCredentialsData>,
}

impl PgCredentials {
    fn to_creds(creds: PgCredentials) -> (String, Creds) {
        let id = creds.id;

        let creds = match (creds.kind, creds.data.0) {
            (PgCredentialsKind::Password, PgCredentialsData::Password(hash)) => {
                Password::from_hash(hash).into()
            }
        };

        (id, creds)
    }
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "text")]
#[sqlx(rename_all = "lowercase")]
enum PgCredentialsKind {
    Password,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum PgCredentialsData {
    Password(String),
}

impl From<&Creds> for PgCredentialsData {
    fn from(creds: &Creds) -> Self {
        match creds {
            Creds::Password(password) => Self::Password(password.password_hash.clone()),
        }
    }
}

#[cfg(test)]
mod tests {}
