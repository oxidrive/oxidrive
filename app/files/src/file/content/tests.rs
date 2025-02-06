use assert2::{check, let_assert};
use bytes::BytesMut;
use futures::{StreamExt, TryStreamExt};

use crate::file;

use super::FileStorage;

async fn upload_and_download_a_file(storage: FileStorage) {
    let owner = oxidrive_accounts::account::fixtures::account();
    let file = file::fixtures::file(owner);

    // It creates a new file
    let data = "hello world";
    let content = file::fixtures::content(data).boxed();

    let size = storage.upload(&file, content).await.unwrap();
    check!(size == data.len());

    let downloaded = storage.download(&file).await.unwrap().unwrap();

    let downloaded: BytesMut = downloaded.try_collect().await.unwrap();
    check!(downloaded.freeze() == data);

    // It updates the same file
    let data = "updated";
    let content = file::fixtures::content(data).boxed();

    let size = storage.upload(&file, content).await.unwrap();
    check!(size == data.len());

    let downloaded = storage.download(&file).await.unwrap().unwrap();

    let downloaded: BytesMut = downloaded.try_collect().await.unwrap();
    check!(downloaded.freeze() == data);
}

async fn download_a_file_that_does_not_exist(storage: FileStorage) {
    let owner = oxidrive_accounts::account::fixtures::account();
    let file = file::fixtures::file(owner);

    let found = storage.download(&file).await.unwrap();
    let_assert!(None = found);
}

mod inmemory {
    use super::*;

    #[tokio::test]
    async fn it_uploads_and_downloads_a_file() {
        let store = FileStorage::memory();
        upload_and_download_a_file(store).await;
    }

    #[tokio::test]
    async fn it_does_not_download_a_file_that_does_not_exist() {
        let store = FileStorage::memory();
        download_a_file_that_does_not_exist(store).await;
    }
}

mod fs {
    use super::*;

    #[tokio::test]
    async fn it_uploads_and_downloads_a_file() {
        let root_dir = tempfile::tempdir().unwrap();

        let store = FileStorage::file_system(root_dir);
        upload_and_download_a_file(store).await;
    }

    #[tokio::test]
    async fn it_does_not_download_a_file_that_does_not_exist() {
        let root_dir = tempfile::tempdir().unwrap();

        let store = FileStorage::file_system(root_dir);
        download_a_file_that_does_not_exist(store).await;
    }
}
