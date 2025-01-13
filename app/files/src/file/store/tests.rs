use assert2::check;
use oxidrive_auth::{
    account::{Account, AccountId},
    account_id,
};
use oxidrive_paginate::Paginate;

use crate::{file, FileId};

use super::FileMetadata;

macro_rules! check_file {
    ($expected:ident, $actual:ident) => {
        check!($expected.id == $actual.id);
        check!($expected.owner_id == $actual.owner_id);
        check!($expected.name == $actual.name);
        check!($expected.size == $actual.size);
        check!($expected.tags == $actual.tags);
    };
}

const FILE_ID_1: FileId = file::file_id!("019433e9-ffbb-7c8b-af6c-d4cb061fb919");
const FILE_ID_2: FileId = file::file_id!("019433ea-5976-7982-bedb-760ad14d4c1a");

const OWNER_ID: AccountId = account_id!("0194327d-becc-7ef3-809c-35dd09f62f45");

fn owner() -> Account {
    Account {
        id: OWNER_ID,
        username: "admin".into(),
    }
}

async fn list_all_files<S: FileMetadata>(store: S) {
    let owner = owner();

    let forward = store
        .all_owned_by(owner.id, Paginate::first(2))
        .await
        .unwrap();
    check!(forward.len() == 2);

    let backward = store
        .all_owned_by(owner.id, Paginate::last(2))
        .await
        .unwrap();
    check!(backward.len() == 2);

    let forward_ids = forward.items.iter().map(|f| f.id).collect::<Vec<_>>();
    let backward_ids = backward.items.iter().map(|f| f.id).collect::<Vec<_>>();

    check!(forward_ids == backward_ids);
}

async fn store_and_load_file_by_id<S: FileMetadata>(store: S) {
    let owner = owner();

    let file = file::fixtures::file(owner.clone());

    let stored = store.save(file.clone()).await.unwrap();
    check_file!(file, stored);

    let loaded = store.by_id(owner.id, file.id).await.unwrap().unwrap();
    check_file!(file, loaded);
}

async fn store_and_load_file_by_name<S: FileMetadata>(store: S) {
    let owner = owner();

    let file = file::fixtures::file(owner.clone());

    let stored = store.save(file.clone()).await.unwrap();
    check_file!(file, stored);

    let loaded = store.by_name(owner.id, &file.name).await.unwrap().unwrap();
    check_file!(file, loaded);
}

const SEARCH_FILES_CASES: &[(&str, &[FileId])] = &[
    ("*", &[FILE_ID_1, FILE_ID_2]),
    ("name content_type:text/plain", &[FILE_ID_1, FILE_ID_2]),
    ("name:hello.txt", &[FILE_ID_1]),
    ("name:world.txt", &[FILE_ID_2]),
    ("file1", &[FILE_ID_1]),
    ("file2", &[FILE_ID_2]),
];

async fn search_files<S: FileMetadata>(store: S) {
    let owner = owner();

    for (query, expected_ids) in SEARCH_FILES_CASES {
        let filter = oxidrive_search::parse_query(query).unwrap();

        let files = store
            .search(owner.id, filter, Paginate::default())
            .await
            .unwrap()
            .items;

        let mut ids = files.into_iter().map(|f| f.id).collect::<Vec<_>>();
        ids.sort();

        check!(*expected_ids == ids.as_slice(), "query failed: {query}");
    }
}

mod inmemory {
    use file::File;

    use crate::{file::InMemoryFileMetadata, tag};

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
    async fn it_stores_and_loads_file_by_id() {
        let store = InMemoryFileMetadata::default();
        store_and_load_file_by_id(store).await;
    }

    #[tokio::test]
    async fn it_stores_and_loads_file_by_name() {
        let store = InMemoryFileMetadata::default();
        store_and_load_file_by_name(store).await;
    }

    #[tokio::test]
    async fn it_searches_files() {
        let mut file_1 = File::new(OWNER_ID, "hello.txt", "text/plain").tagged(tag!("file1"));
        file_1.id = FILE_ID_1;

        let mut file_2 = File::new(OWNER_ID, "world.txt", "text/plain").tagged(tag!("file2"));
        file_2.id = FILE_ID_2;

        let store = InMemoryFileMetadata::from([file_1, file_2]);
        search_files(store).await;
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
    async fn it_stores_and_loads_file_by_id(pool: sqlx::PgPool) {
        let store = PgFileMetadata::new(pool);
        store_and_load_file_by_id(store).await;
    }

    #[sqlx::test(
        migrator = "PG_MIGRATOR",
        fixtures("../../fixtures/postgres/accounts.sql")
    )]
    async fn it_stores_and_loads_file_by_name(pool: sqlx::PgPool) {
        let store = PgFileMetadata::new(pool);
        store_and_load_file_by_name(store).await;
    }

    #[sqlx::test(
        migrator = "PG_MIGRATOR",
        fixtures(
            "../../fixtures/postgres/accounts.sql",
            "../../fixtures/postgres/files.sql"
        )
    )]
    async fn it_searches_files(pool: sqlx::PgPool) {
        let store = PgFileMetadata::new(pool);
        search_files(store).await;
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
    async fn it_stores_and_loads_file_by_id(pool: sqlx::SqlitePool) {
        let store = SqliteFileMetadata::new(pool);
        store_and_load_file_by_id(store).await;
    }

    #[sqlx::test(
        migrator = "SQLITE_MIGRATOR",
        fixtures("../../fixtures/sqlite/accounts.sql",)
    )]
    async fn it_stores_and_loads_file_by_name(pool: sqlx::SqlitePool) {
        let store = SqliteFileMetadata::new(pool);
        store_and_load_file_by_name(store).await;
    }

    #[sqlx::test(
        migrator = "SQLITE_MIGRATOR",
        fixtures(
            "../../fixtures/sqlite/accounts.sql",
            "../../fixtures/sqlite/files.sql"
        )
    )]
    async fn it_searches_files(pool: sqlx::SqlitePool) {
        let store = SqliteFileMetadata::new(pool);
        search_files(store).await;
    }
}
