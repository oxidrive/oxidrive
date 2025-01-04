use assert2::check;

use super::*;
use crate::account::{credentials, Account};

async fn store_and_load_credentials_for_a_user<S: AccountCredentials>(store: S) {
    let account = Account {
        id: "0194327d-becc-7ef3-809c-35dd09f62f45".parse().unwrap(),
        username: "admin".into(),
    };

    let credentials = credentials::fixtures::with_password(account.clone(), "password".into());

    let stored = store.save(credentials).await.unwrap();
    check!(stored.account_id == account.id);
    check!(!stored.creds.is_empty());

    let loaded = store.for_account(account.id).await.unwrap();
    check!(loaded.account_id == account.id);
    check!(stored.account_id == loaded.account_id);
    check!(stored.creds == loaded.creds);
    check!(!loaded.creds.is_empty());

    let credentials = credentials::fixtures::empty(account.clone());

    let empty = store.save(credentials).await.unwrap();
    check!(empty.account_id == account.id);
    check!(empty.creds.is_empty());

    let loaded = store.for_account(account.id).await.unwrap();
    check!(loaded.account_id == account.id);
    check!(stored.account_id == loaded.account_id);
    check!(stored.creds == loaded.creds);
    check!(!loaded.creds.is_empty());
}

mod inmemory {
    use super::*;

    #[tokio::test]
    async fn it_stores_and_loads_credentials_for_a_user() {
        let store = InMemoryCredentials::default();
        store_and_load_credentials_for_a_user(store).await;
    }
}

mod pg {
    use oxidrive_database::migrate::PG_MIGRATOR;

    use super::*;

    #[sqlx::test(
        migrator = "PG_MIGRATOR",
        fixtures("../../../fixtures/postgres/accounts.sql")
    )]
    async fn it_stores_and_loads_credentials_for_a_user(pool: sqlx::PgPool) {
        let store = PgAccountCredentials::new(pool);
        store_and_load_credentials_for_a_user(store).await;
    }
}

mod sqlite {
    use oxidrive_database::migrate::SQLITE_MIGRATOR;

    use super::*;

    #[sqlx::test(
        migrator = "SQLITE_MIGRATOR",
        fixtures("../../../fixtures/sqlite/accounts.sql")
    )]
    async fn it_stores_and_loads_credentials_for_a_user(pool: sqlx::SqlitePool) {
        let store = SqliteAccountCredentials::new(pool);
        store_and_load_credentials_for_a_user(store).await;
    }
}
