use async_trait::async_trait;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::session::{Session, SessionId};

use super::{ByIdError, DeleteExpiredError, DeleteSessionError, SaveSessionError, SessionStore};

#[derive(Clone)]
pub struct PgSessions {
    pool: PgPool,
}

impl PgSessions {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SessionStore for PgSessions {
    async fn by_id(&self, id: SessionId) -> Result<Option<Session>, ByIdError> {
        let session: Option<PgSession> = sqlx::query_as(
            r#"
select
  id,
  account_id,
  expires_at
from sessions
where id = $1
  and expires_at >= $2
"#,
        )
        .bind(id.as_uuid())
        .bind(OffsetDateTime::now_utc())
        .fetch_optional(&self.pool)
        .await
        .map_err(ByIdError::wrap)?;

        Ok(session.map(Into::into))
    }

    async fn save(&self, session: Session) -> Result<Session, SaveSessionError> {
        sqlx::query(
            r#"
insert into sessions (
  id,
  account_id,
  expires_at
) values (
  $1,
  $2,
  $3
)
"#,
        )
        .bind(session.id.as_uuid())
        .bind(session.account_id.as_uuid())
        .bind(session.expires_at)
        .execute(&self.pool)
        .await
        .map_err(SaveSessionError::wrap)?;

        Ok(session)
    }

    async fn delete(&self, id: SessionId) -> Result<(), DeleteSessionError> {
        sqlx::query("delete from sessions where id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(DeleteSessionError::wrap)?;
        Ok(())
    }

    async fn delete_expired(&self) -> Result<Vec<Session>, DeleteExpiredError> {
        let deleted: Vec<PgSession> = sqlx::query_as(
            r#"
delete from sessions
where expires_at < $1
returning
  id,
  account_id,
  expires_at
"#,
        )
        .bind(OffsetDateTime::now_utc())
        .fetch_all(&self.pool)
        .await
        .map_err(DeleteExpiredError::wrap)?;

        Ok(deleted.into_iter().map(Into::into).collect())
    }
}

#[derive(sqlx::FromRow)]
struct PgSession {
    id: Uuid,
    account_id: Uuid,
    expires_at: OffsetDateTime,
}

impl From<PgSession> for Session {
    fn from(session: PgSession) -> Self {
        Self {
            id: session.id.into(),
            account_id: session.account_id.into(),
            expires_at: session.expires_at,
        }
    }
}
