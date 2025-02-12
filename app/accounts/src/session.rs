use std::sync::Arc;

use jobs::JobsModule;
use oxidrive_database::Database;
use oxidrive_domain::make_uuid_type;
use time::{Duration, OffsetDateTime};

use crate::account::AccountId;

pub use service::*;
pub use store::*;

pub mod jobs;
mod service;
mod store;

static DEFAULT_SESSION_DURATION: Duration = Duration::weeks(1);

make_uuid_type!(SessionId, session_id, new_v4);

#[derive(Clone, Debug)]
pub struct Session {
    pub id: SessionId,
    pub account_id: AccountId,
    pub expires_at: OffsetDateTime,
}

impl Session {
    pub fn create(account_id: AccountId) -> Self {
        Self {
            id: SessionId::new(),
            account_id,
            expires_at: OffsetDateTime::now_utc() + DEFAULT_SESSION_DURATION,
        }
    }
}

#[derive(Copy, Clone)]
pub struct SessionsModule;

impl app::Module for SessionsModule {
    fn mount(self: Box<Self>, c: &mut app::di::Context) {
        c.bind(store);
        c.bind(Sessions::new);
    }
}

fn store(database: Database) -> Arc<dyn SessionStore> {
    match database {
        Database::Sqlite(pool) => Arc::new(SqliteSessions::new(pool)),
        Database::Pg(pool) => Arc::new(PgSessions::new(pool)),
    }
}

#[app::async_trait]
impl app::Hooks for SessionsModule {
    async fn before_start(&mut self, c: &app::di::Container) -> app::eyre::Result<()> {
        JobsModule.before_start(c).await?;
        Ok(())
    }

    async fn after_start(&mut self, c: &app::di::Container) -> app::eyre::Result<()> {
        JobsModule.after_start(c).await?;
        Ok(())
    }

    async fn on_shutdown(&mut self, c: &app::di::Container) -> app::eyre::Result<()> {
        JobsModule.on_shutdown(c).await?;
        Ok(())
    }
}
