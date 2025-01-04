use assert2::check;
use bytes::BytesMut;
use futures::{StreamExt, TryStreamExt};

use crate::file;

use super::FileContents;

async fn upload_and_download_a_file<S: FileContents>(store: S) {
    let owner = oxidrive_auth::account::fixtures::account();
    let file = file::fixtures::file(owner);
    let content = file::fixtures::content("hello world").boxed();

    store.upload(&file, content).await.unwrap();

    let downloaded = store
        .download(file.owner_id, &file.name)
        .await
        .unwrap()
        .unwrap();

    let downloaded: BytesMut = downloaded.try_collect().await.unwrap();
    check!(downloaded.freeze() == "hello world");
}

mod inmemory {
    use file::InMemoryFileContents;

    use super::*;

    #[tokio::test]
    async fn it_uploads_and_downloads_a_file() {
        let store = InMemoryFileContents::default();
        upload_and_download_a_file(store).await;
    }
}

mod fs {

    use file::FsFileContents;

    use super::*;

    #[tokio::test]
    async fn it_uploads_and_downloads_a_file() {
        let root_dir = tempfile::tempdir().unwrap();

        let store = FsFileContents::new(root_dir);
        upload_and_download_a_file(store).await;
    }
}
