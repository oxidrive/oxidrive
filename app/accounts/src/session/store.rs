use async_trait::async_trait;
use oxidrive_domain::make_error_wrapper;

use super::{Session, SessionId};

pub use pg::PgSessions;
pub use sqlite::SqliteSessions;

mod pg;
mod sqlite;

make_error_wrapper!(ByIdError);
make_error_wrapper!(SaveSessionError);
make_error_wrapper!(DeleteSessionError);
make_error_wrapper!(DeleteExpiredError);

#[mockall::automock]
#[async_trait]
pub trait SessionStore: Send + Sync + 'static {
    async fn by_id(&self, id: SessionId) -> Result<Option<Session>, ByIdError>;
    async fn save(&self, session: Session) -> Result<Session, SaveSessionError>;
    async fn delete(&self, id: SessionId) -> Result<(), DeleteSessionError>;
    async fn delete_expired(&self) -> Result<Vec<Session>, DeleteExpiredError>;
}

#[cfg(test)]
mod tests;
