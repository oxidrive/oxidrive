use async_trait::async_trait;
use sqlx::SqlitePool;
use time::OffsetDateTime;

use crate::session::{Session, SessionId};

use super::{ByIdError, DeleteExpiredError, DeleteSessionError, SaveSessionError, SessionStore};

#[derive(Clone)]
pub struct SqliteSessions {
    pool: SqlitePool,
}

impl SqliteSessions {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SessionStore for SqliteSessions {
    async fn by_id(&self, id: SessionId) -> Result<Option<Session>, ByIdError> {
        let session: Option<SqliteSession> = sqlx::query_as(
            r#"
select
  id,
  account_id,
  expires_at
from sessions
where id = ?
  and expires_at >= ?
"#,
        )
        .bind(id.to_string())
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
  ?,
  ?,
  ?
)
"#,
        )
        .bind(session.id.to_string())
        .bind(session.account_id.to_string())
        .bind(session.expires_at)
        .execute(&self.pool)
        .await
        .map_err(SaveSessionError::wrap)?;

        Ok(session)
    }

    async fn delete(&self, id: SessionId) -> Result<(), DeleteSessionError> {
        sqlx::query("delete from sessions where id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(DeleteSessionError::wrap)?;
        Ok(())
    }

    async fn delete_expired(&self) -> Result<Vec<Session>, DeleteExpiredError> {
        let deleted: Vec<SqliteSession> = sqlx::query_as(
            r#"
delete from sessions
where expires_at < ?
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
struct SqliteSession {
    id: String,
    account_id: String,
    expires_at: OffsetDateTime,
}

impl From<SqliteSession> for Session {
    fn from(session: SqliteSession) -> Self {
        Self {
            id: session.id.parse().unwrap(),
            account_id: session.account_id.parse().unwrap(),
            expires_at: session.expires_at,
        }
    }
}
