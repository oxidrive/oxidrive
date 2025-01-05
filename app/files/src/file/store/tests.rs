use assert2::check;
use oxidrive_auth::account::Account;
use oxidrive_paginate::Paginate;

use crate::file;

use super::FileMetadata;

fn owner() -> Account {
    Account {
        id: "0194327d-becc-7ef3-809c-35dd09f62f45".parse().unwrap(),
        username: "admin".into(),
    }
}

async fn list_all_files<S: FileMetadata>(store: S) {
    let owner = owner();

    let mut forward = store
        .all_owned_by(owner.id, Paginate::first(2))
        .await
        .unwrap();
    check!(forward.len() == 2);

    let backward = store
        .all_owned_by(owner.id, Paginate::last(2))
        .await
        .unwrap();
    check!(backward.len() == 2);

    forward.items.reverse();
    let forward_ids = forward.items.iter().map(|f| f.id).collect::<Vec<_>>();
    let backward_ids = backward.items.iter().map(|f| f.id).collect::<Vec<_>>();
    check!(forward_ids == backward_ids);
}

async fn store_and_load_account_by_file_name<S: FileMetadata>(store: S) {
    let owner = owner();

    let file = file::fixtures::file(owner.clone());

    let stored = store.save(file.clone()).await.unwrap();
    check!(stored.id == file.id);

    let loaded = store.by_name(owner.id, &file.name).await.unwrap().unwrap();
    check!(loaded.id == file.id);
    check!(loaded.owner_id == file.owner_id);
    check!(loaded.name == file.name);
}

mod inmemory {
    use crate::file::InMemoryFileMetadata;

    use super::*;

    #[tokio::test]
    async fn it_lists_all_files() {
        let owner = owner();

        let store = InMemoryFileMetadata::from([
            file::fixtures::file(owner.clone()),
            file::fixtures::file(owner),
        ]);
        list_all_files(store).await;
    }

    #[tokio::test]
    async fn it_stores_and_loads_account_by_file_name() {
        let store = InMemoryFileMetadata::default();
        store_and_load_account_by_file_name(store).await;
    }
}

mod pg {
    use oxidrive_database::migrate::PG_MIGRATOR;

    use crate::file::PgFileMetadata;

    use super::*;

    #[sqlx::test(
        migrator = "PG_MIGRATOR",
        fixtures(
            "../../fixtures/postgres/accounts.sql",
            "../../fixtures/postgres/files.sql"
        )
    )]
    async fn it_lists_all_files(pool: sqlx::PgPool) {
        let store = PgFileMetadata::new(pool);
        list_all_files(store).await;
    }

    #[sqlx::test(
        migrator = "PG_MIGRATOR",
        fixtures("../../fixtures/postgres/accounts.sql")
    )]
    async fn it_stores_and_loads_account_by_file_name(pool: sqlx::PgPool) {
        let store = PgFileMetadata::new(pool);
        store_and_load_account_by_file_name(store).await;
    }
}

mod sqlite {
    use oxidrive_database::migrate::SQLITE_MIGRATOR;

    use crate::file::SqliteFileMetadata;

    use super::*;

    #[sqlx::test(
        migrator = "SQLITE_MIGRATOR",
        fixtures(
            "../../fixtures/sqlite/accounts.sql",
            "../../fixtures/sqlite/files.sql"
        )
    )]
    async fn it_lists_all_files(pool: sqlx::SqlitePool) {
        let store = SqliteFileMetadata::new(pool);
        list_all_files(store).await;
    }

    #[sqlx::test(
        migrator = "SQLITE_MIGRATOR",
        fixtures("../../fixtures/sqlite/accounts.sql",)
    )]
    async fn it_stores_and_loads_account_by_file_name(pool: sqlx::SqlitePool) {
        let store = SqliteFileMetadata::new(pool);
        store_and_load_account_by_file_name(store).await;
    }
}
