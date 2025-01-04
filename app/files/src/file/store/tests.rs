use assert2::check;
use oxidrive_auth::account::Account;

use crate::file;

use super::FileMetadata;

async fn store_and_load_account_by_file_name<S: FileMetadata>(store: S) {
    let owner = Account {
        id: "01943350-aacf-7b8c-b45f-b0f5f220ab93".parse().unwrap(),
        username: "admin".into(),
    };

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
        fixtures("../../fixtures/sqlite/accounts.sql")
    )]
    async fn it_stores_and_loads_account_by_file_name(pool: sqlx::SqlitePool) {
        let store = SqliteFileMetadata::new(pool);
        store_and_load_account_by_file_name(store).await;
    }
}
