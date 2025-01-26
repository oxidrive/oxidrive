use assert2::check;

use crate::account;

use super::Accounts;

async fn store_and_load_account_by_id<S: Accounts>(store: S) {
    let account = account::fixtures::account();

    let stored = store.save(account.clone()).await.unwrap();
    check!(stored.id == account.id);

    let loaded = store.by_id(account.id).await.unwrap().unwrap();
    check!(loaded.id == account.id);
    check!(loaded.username == account.username);
}

async fn store_and_load_account_by_username<S: Accounts>(store: S) {
    let account = account::fixtures::account();

    let stored = store.save(account.clone()).await.unwrap();
    check!(stored.id == account.id);

    let loaded = store.by_username(&account.username).await.unwrap().unwrap();
    check!(loaded.id == account.id);
    check!(loaded.username == account.username);
}

async fn count_accounts<S: Accounts>(store: S, expected: usize) {
    let actual = store.count().await.unwrap();
    check!(actual == expected);
}

mod inmemory {
    use crate::account::InMemoryAccounts;

    use super::*;

    #[tokio::test]
    async fn it_stores_and_loads_an_account_by_id() {
        let store = InMemoryAccounts::default();
        store_and_load_account_by_id(store).await;
    }

    #[tokio::test]
    async fn it_stores_and_loads_an_account_by_username() {
        let store = InMemoryAccounts::default();
        store_and_load_account_by_username(store).await;
    }

    #[tokio::test]
    async fn it_counts_accounts() {
        let store = InMemoryAccounts::from([
            account::fixtures::account(),
            account::fixtures::account(),
            account::fixtures::account(),
        ]);
        count_accounts(store, 3).await;
    }
}

mod pg {
    use oxidrive_database::migrate::PG_MIGRATOR;

    use crate::account::PgAccounts;

    use super::*;

    #[sqlx::test(migrator = "PG_MIGRATOR")]
    async fn it_stores_and_loads_an_account_by_id(pool: sqlx::PgPool) {
        let store = PgAccounts::new(pool);
        store_and_load_account_by_id(store).await;
    }

    #[sqlx::test(migrator = "PG_MIGRATOR")]
    async fn it_stores_and_loads_an_account_by_username(pool: sqlx::PgPool) {
        let store = PgAccounts::new(pool);
        store_and_load_account_by_username(store).await;
    }

    #[sqlx::test(
        migrator = "PG_MIGRATOR",
        fixtures("../../fixtures/postgres/accounts.sql")
    )]
    async fn it_counts_accounts(pool: sqlx::PgPool) {
        let store = PgAccounts::new(pool);
        count_accounts(store, 3).await;
    }
}

mod sqlite {
    use oxidrive_database::migrate::SQLITE_MIGRATOR;

    use crate::account::SqliteAccounts;

    use super::*;

    #[sqlx::test(migrator = "SQLITE_MIGRATOR")]
    async fn it_stores_and_loads_an_account_by_id(pool: sqlx::SqlitePool) {
        let store = SqliteAccounts::new(pool);
        store_and_load_account_by_id(store).await;
    }

    #[sqlx::test(migrator = "SQLITE_MIGRATOR")]
    async fn it_stores_and_loads_an_account_by_username(pool: sqlx::SqlitePool) {
        let store = SqliteAccounts::new(pool);
        store_and_load_account_by_username(store).await;
    }

    #[sqlx::test(
        migrator = "SQLITE_MIGRATOR",
        fixtures("../../fixtures/postgres/accounts.sql")
    )]
    async fn it_counts_accounts(pool: sqlx::SqlitePool) {
        let store = SqliteAccounts::new(pool);
        count_accounts(store, 3).await;
    }
}
