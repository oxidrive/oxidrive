use assert2::{assert, check};
use oxidrive_auth::{account::Account, account_id};
use oxidrive_workers::Process;

use crate::{
    collection::{
        self,
        jobs::{RefreshCollection, RefreshCollectionWorker},
    },
    file::{
        self,
        store::tests::{FILE_ID_1, FILE_ID_2},
        FileMetadata,
    },
};

use super::*;

const COLLECTION_ID_1: CollectionId =
    collection::macros::collection_id!("019497f6-6111-70c5-8575-420fff86e99b");
const COLLECTION_ID_2: CollectionId =
    collection::macros::collection_id!("019497f7-6a4b-7226-9a3c-391628318f5c");

fn collection_1() -> Collection {
    Collection {
        id: COLLECTION_ID_1,
        name: "All Files".into(),
        owner_id: OWNER_ID,
        filter: "*".parse().unwrap(),
        files: [FILE_ID_1, FILE_ID_2].into(),
    }
}

fn collection_2() -> Collection {
    Collection {
        id: COLLECTION_ID_2,
        name: "Text Files".into(),
        owner_id: OWNER_ID,
        filter: "ext:txt OR content_type:text/plain".parse().unwrap(),
        files: [FILE_ID_1, FILE_ID_2].into(),
    }
}

const OWNER_ID: AccountId = account_id!("0194327d-becc-7ef3-809c-35dd09f62f45");

fn owner() -> Account {
    Account {
        id: OWNER_ID,
        username: "admin".into(),
    }
}

macro_rules! check_collection_eq {
    ($actual:expr, $expected:expr) => {
        check!($actual.id == $expected.id);
        check!($actual.name == $expected.name);
        check!($actual.owner_id == $expected.owner_id);
        check!($actual.filter == $expected.filter);
        check!($actual.files == $expected.files);
    };
}

async fn list_collections<S: CollectionStore, F: FileMetadata>(store: S, files: F) {
    let store = Arc::new(store);
    let refresh = RefreshCollectionWorker::new(Arc::new(files), store.clone());

    refresh
        .process(RefreshCollection {
            collection_id: COLLECTION_ID_1,
        })
        .await
        .unwrap();
    refresh
        .process(RefreshCollection {
            collection_id: COLLECTION_ID_2,
        })
        .await
        .unwrap();

    let collections = store
        .all_owned_by(OWNER_ID, Paginate::default())
        .await
        .unwrap();

    assert!(collections.items.len() == 2);

    let mut collections = collections.iter();

    let c1 = collections.next().unwrap();
    check_collection_eq!(c1, collection_1());

    let c2 = collections.next().unwrap();
    check_collection_eq!(c2, collection_2());
}

async fn store_and_fetch_by_id<S: CollectionStore>(store: S) {
    let owner = owner();

    let mut collection = collection::fixtures::collection(owner);
    collection.add([FILE_ID_1]);

    let saved = store.save(collection.clone()).await.unwrap();
    check_collection_eq!(saved, collection);

    let found = store.by_id(collection.id).await.unwrap().unwrap();
    check_collection_eq!(found, collection);
}

mod inmemory {
    use file::{
        store::tests::{file_1, file_2},
        InMemoryFileMetadata,
    };

    use super::*;

    #[tokio::test]
    async fn it_lists_collections() {
        let store = InMemoryCollectionStore::from([collection_1(), collection_2()]);
        let files = InMemoryFileMetadata::from([file_1(), file_2()]);
        list_collections(store, files).await;
    }

    #[tokio::test]
    async fn it_stores_and_fetches_a_collection_by_id() {
        let store = InMemoryCollectionStore::default();
        store_and_fetch_by_id(store).await;
    }
}

mod pg {
    use file::PgFileMetadata;
    use oxidrive_database::migrate::PG_MIGRATOR;

    use super::*;

    #[sqlx::test(
        migrator = "PG_MIGRATOR",
        fixtures(
            "../../fixtures/postgres/accounts.sql",
            "../../fixtures/postgres/files.sql",
            "../../fixtures/postgres/collections.sql"
        )
    )]
    async fn it_lists_collections(pool: sqlx::PgPool) {
        let store = PgCollectionStore::new(pool.clone());
        let files = PgFileMetadata::new(pool);
        list_collections(store, files).await;
    }

    #[sqlx::test(
        migrator = "PG_MIGRATOR",
        fixtures(
            "../../fixtures/postgres/accounts.sql",
            "../../fixtures/postgres/files.sql",
        )
    )]
    async fn it_stores_and_fetches_a_collection_by_id(pool: sqlx::PgPool) {
        let store = PgCollectionStore::new(pool);
        store_and_fetch_by_id(store).await;
    }
}

mod sqlite {
    use file::SqliteFileMetadata;
    use oxidrive_database::migrate::SQLITE_MIGRATOR;

    use super::*;

    #[sqlx::test(
        migrator = "SQLITE_MIGRATOR",
        fixtures(
            "../../fixtures/sqlite/accounts.sql",
            "../../fixtures/sqlite/files.sql",
            "../../fixtures/sqlite/collections.sql"
        )
    )]
    async fn it_lists_collections(pool: sqlx::SqlitePool) {
        let store = SqliteCollectionStore::new(pool.clone());
        let files = SqliteFileMetadata::new(pool);
        list_collections(store, files).await;
    }

    #[sqlx::test(
        migrator = "SQLITE_MIGRATOR",
        fixtures(
            "../../fixtures/sqlite/accounts.sql",
            "../../fixtures/sqlite/files.sql"
        )
    )]
    async fn it_stores_and_fetches_a_collection_by_id(pool: sqlx::SqlitePool) {
        let store = SqliteCollectionStore::new(pool);
        store_and_fetch_by_id(store).await;
    }
}
