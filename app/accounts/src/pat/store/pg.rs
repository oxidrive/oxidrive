use async_trait::async_trait;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::pat::{PersonalAccessToken, Token};

use super::{ByTokenError, PersonalAccessTokenStore, SaveError};

pub struct PgPersonalAccessTokens {
    pool: PgPool,
}

impl PgPersonalAccessTokens {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PersonalAccessTokenStore for PgPersonalAccessTokens {
    async fn by_token(&self, token: Token) -> Result<Option<PersonalAccessToken>, ByTokenError> {
        let token: Option<PgPersonalAccessToken> = sqlx::query_as(
            r#"
select
  id,
  token_hash,
  account_id,
  expires_at
from personal_access_tokens
where token_hash = $1
and (expires_at is null or expires_at > $2)
"#,
        )
        .bind(token.hashed().as_bytes())
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
  $1,
  $2,
  $3,
  $4
)
"#,
        )
        .bind(token.id.as_uuid())
        .bind(token.token_hash.as_bytes())
        .bind(token.account_id.as_uuid())
        .bind(token.expires_at)
        .execute(&self.pool)
        .await
        .map_err(SaveError::wrap)?;

        Ok(token)
    }
}

#[derive(sqlx::FromRow)]
struct PgPersonalAccessToken {
    id: Uuid,
    token_hash: Vec<u8>,
    account_id: Uuid,
    expires_at: Option<OffsetDateTime>,
}

impl From<PgPersonalAccessToken> for PersonalAccessToken {
    fn from(pat: PgPersonalAccessToken) -> Self {
        Self {
            id: pat.id.into(),
            token_hash: blake3::Hash::from_bytes(pat.token_hash.try_into().unwrap()),
            account_id: pat.account_id.into(),
            expires_at: pat.expires_at,
        }
    }
}
