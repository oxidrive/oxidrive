use assert2::{check, let_assert};
use oxidrive_auth::account;
use oxidrive_files::file;

use super::*;

async fn index_a_file_with_some_tags<S: TagIndex>(index: S) {
    let owner = account::fixtures::account();
    let file = file::fixtures::file(owner);

    let mut tags = vec![
        Tag::parse("example").unwrap(),
        Tag::parse("hello:world").unwrap(),
    ];

    index.index(&file, tags.clone()).await.unwrap();

    // adding the same tag twice is a noop
    index.index(&file, tags.clone()).await.unwrap();

    let mut tags = index.for_file(file.id).await.unwrap();
    tags.sort();

    check!(tags.len() == 2);
    let_assert!(Some(found) = tags.first());
    check!(*found == "example");
    let_assert!(Some(found) = tags.get(1));
    check!(*found == "hello:world");

    // adding a new tag amongst existing tags skips the duplicates
    tags.push(Tag::parse("test").unwrap());

    index.index(&file, tags.clone()).await.unwrap();

    let mut tags = index.for_file(file.id).await.unwrap();
    tags.sort();

    check!(tags.len() == 3);
    let_assert!(Some(found) = tags.first());
    check!(*found == "example");
    let_assert!(Some(found) = tags.get(1));
    check!(*found == "hello:world");
    let_assert!(Some(found) = tags.get(2));
    check!(*found == "test");
}

mod inmemory {
    use super::*;

    #[tokio::test]
    async fn it_indexes_a_file_with_some_tags() {
        let index = InMemoryTagIndex::default();
        index_a_file_with_some_tags(index).await;
    }
}

mod pg {
    use oxidrive_database::migrate::PG_MIGRATOR;

    use crate::tag::PgTagIndex;

    use super::*;

    #[sqlx::test(migrator = "PG_MIGRATOR")]
    async fn it_indexes_a_file_with_some_tags(pool: sqlx::PgPool) {
        let store = PgTagIndex::new(pool);
        index_a_file_with_some_tags(store).await;
    }
}

mod sqlite {
    use oxidrive_database::migrate::SQLITE_MIGRATOR;

    use crate::tag::SqliteTagIndex;

    use super::*;

    #[sqlx::test(migrator = "SQLITE_MIGRATOR")]
    async fn it_indexes_a_file_with_some_tags(pool: sqlx::SqlitePool) {
        let store = SqliteTagIndex::new(pool);
        index_a_file_with_some_tags(store).await;
    }
}
