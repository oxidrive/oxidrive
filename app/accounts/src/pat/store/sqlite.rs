use async_trait::async_trait;
use sqlx::SqlitePool;
use time::OffsetDateTime;

use crate::pat::{PersonalAccessToken, Token};

use super::{ByTokenError, PersonalAccessTokenStore, SaveError};

pub struct SqlitePersonalAccessTokens {
    pool: SqlitePool,
}

impl SqlitePersonalAccessTokens {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PersonalAccessTokenStore for SqlitePersonalAccessTokens {
    async fn by_token(&self, token: Token) -> Result<Option<PersonalAccessToken>, ByTokenError> {
        let token: Option<SqlitePersonalAccessToken> = sqlx::query_as(
            r#"
select
  id,
  token_hash,
  account_id,
  expires_at
from personal_access_tokens
where token_hash = ?
and (expires_at is null or expires_at > ?)
"#,
        )
        .bind(token.hashed().as_bytes().as_slice())
        .bind(OffsetDateTime::now_utc())
        .fetch_optional(&self.pool)
        .await
        .map_err(ByTokenError::wrap)?;

        Ok(token.map(Into::into))
    }

    async fn save(&self, token: PersonalAccessToken) -> Result<PersonalAccessToken, SaveError> {
        sqlx::query(
            r#"
insert into personal_access_tokens (
  id,
  token_hash,
  account_id,
  expires_at
) values (
  ?,
  ?,
  ?,
  ?
)
"#,
        )
        .bind(token.id.to_string())
        .bind(token.token_hash.as_bytes().as_slice())
        .bind(token.account_id.to_string())
        .bind(token.expires_at)
        .execute(&self.pool)
        .await
        .map_err(SaveError::wrap)?;

        Ok(token)
    }
}

#[derive(sqlx::FromRow)]
struct SqlitePersonalAccessToken {
    id: String,
    token_hash: Vec<u8>,
    account_id: String,
    expires_at: Option<OffsetDateTime>,
}

impl From<SqlitePersonalAccessToken> for PersonalAccessToken {
    fn from(pat: SqlitePersonalAccessToken) -> Self {
        Self {
            id: pat.id.parse().unwrap(),
            token_hash: blake3::Hash::from_bytes(pat.token_hash.try_into().unwrap()),
            account_id: pat.account_id.parse().unwrap(),
            expires_at: pat.expires_at,
        }
    }
}
