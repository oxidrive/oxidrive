use crate::{
    account::{macros::account_id, AccountId},
    session::{macros::session_id, Session, SessionId},
};
use assert2::{check, let_assert};
use time::OffsetDateTime;

use super::SessionStore;

fn truncate_to_seconds(ts: OffsetDateTime) -> OffsetDateTime {
    ts.replace_millisecond(0)
        .unwrap()
        .replace_microsecond(0)
        .unwrap()
}

macro_rules! check_session {
    ($expected:expr, $actual:expr) => {
        check!($expected.id == $actual.id);
        check!($expected.account_id == $actual.account_id);
        check!(
            truncate_to_seconds($expected.expires_at) == truncate_to_seconds($actual.expires_at)
        );
    };
}

const SESSION_ID: SessionId = session_id!("2814c74f-699a-4b96-b0ed-cb86425104d4");
const EXPIRED_SESSION_ID: SessionId = session_id!("d65f698d-751f-4a9b-b7a1-ca28b9df8786");

const OWNER_ID: AccountId = account_id!("0194327d-becc-7ef3-809c-35dd09f62f45");

async fn create_a_session<S: SessionStore>(store: S) {
    let session = Session::create(OWNER_ID);

    let saved = store.save(session.clone()).await.unwrap();
    check_session!(session, saved);

    let found = store.by_id(session.id).await.unwrap().unwrap();
    check_session!(session, found);
}

async fn delete_session<S: SessionStore>(store: S) {
    let_assert!(Some(session) = store.by_id(SESSION_ID).await.unwrap());

    store.delete(session.id).await.unwrap();

    let_assert!(None = store.by_id(SESSION_ID).await.unwrap());
}

async fn delete_expired_sessions<S: SessionStore>(store: S) {
    let_assert!(Some(_) = store.by_id(SESSION_ID).await.unwrap());
    let_assert!(None = store.by_id(EXPIRED_SESSION_ID).await.unwrap());

    let deleted = store.delete_expired().await.unwrap();
    check!(deleted.len() == 1);
    let_assert!(Some(deleted) = deleted.first());
    check!(deleted.id == EXPIRED_SESSION_ID);
}

mod pg {
    use oxidrive_database::migrate::PG_MIGRATOR;

    use crate::session::store::pg::PgSessions;

    use super::*;

    #[sqlx::test(
        migrator = "PG_MIGRATOR",
        fixtures("../../fixtures/postgres/accounts.sql")
    )]
    async fn it_creates_a_session(pool: sqlx::PgPool) {
        let store = PgSessions::new(pool);
        create_a_session(store).await;
    }

    #[sqlx::test(
        migrator = "PG_MIGRATOR",
        fixtures(
            "../../fixtures/postgres/accounts.sql",
            "../../fixtures/postgres/sessions.sql"
        )
    )]
    async fn it_deletes_a_session(pool: sqlx::PgPool) {
        let store = PgSessions::new(pool);
        delete_session(store).await;
    }

    #[sqlx::test(
        migrator = "PG_MIGRATOR",
        fixtures(
            "../../fixtures/postgres/accounts.sql",
            "../../fixtures/postgres/sessions.sql"
        )
    )]
    async fn it_deletes_expired_sessions(pool: sqlx::PgPool) {
        let store = PgSessions::new(pool);
        delete_expired_sessions(store).await;
    }
}

mod sqlite {
    use oxidrive_database::migrate::SQLITE_MIGRATOR;

    use crate::session::store::sqlite::SqliteSessions;

    use super::*;
    #[sqlx::test(
        migrator = "SQLITE_MIGRATOR",
        fixtures("../../fixtures/sqlite/accounts.sql")
    )]
    async fn it_creates_a_session(pool: sqlx::SqlitePool) {
        let store = SqliteSessions::new(pool);
        create_a_session(store).await;
    }

    #[sqlx::test(
        migrator = "SQLITE_MIGRATOR",
        fixtures(
            "../../fixtures/sqlite/accounts.sql",
            "../../fixtures/sqlite/sessions.sql"
        )
    )]
    async fn it_deletes_a_session(pool: sqlx::SqlitePool) {
        let store = SqliteSessions::new(pool);
        delete_session(store).await;
    }

    #[sqlx::test(
        migrator = "SQLITE_MIGRATOR",
        fixtures(
            "../../fixtures/sqlite/accounts.sql",
            "../../fixtures/sqlite/sessions.sql"
        )
    )]
    async fn it_deletes_expired_sessions(pool: sqlx::SqlitePool) {
        let store = SqliteSessions::new(pool);
        delete_expired_sessions(store).await;
    }
}
