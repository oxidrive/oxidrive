use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use async_trait::async_trait;
use bytes::{Buf, Bytes, BytesMut};
use futures::{stream::BoxStream, StreamExt, TryStreamExt};
use oxidrive_accounts::account::AccountId;
use tokio::{fs, io::AsyncWriteExt};
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::File;

use super::{ContentStreamError, DownloadFileError, FileContents, UploadFileError};

pub struct FsFileContents {
    root_dir: PathBuf,
}

impl FsFileContents {
    pub fn new(root_dir: impl AsRef<Path>) -> Self {
        Self {
            root_dir: root_dir.as_ref().to_path_buf(),
        }
    }

    fn path_for(&self, owner_id: AccountId, file_name: impl AsRef<Path>) -> PathBuf {
        self.root_dir.join(owner_id.to_string()).join(file_name)
    }

    fn path_for_file(&self, file: &File) -> PathBuf {
        self.path_for(file.owner_id, file.id.to_string())
    }
}

#[async_trait]
impl FileContents for FsFileContents {
    fn display_name(&self) -> Cow<'static, str> {
        "FileSystem".into()
    }

    async fn download(
        &self,
        owner_id: AccountId,
        file_name: &str,
    ) -> Result<Option<BoxStream<'static, Result<Bytes, ContentStreamError>>>, DownloadFileError>
    {
        let path = self.path_for(owner_id, file_name);

        let file = match fs::File::open(path).await {
            Ok(file) => file,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(DownloadFileError::wrap(err)),
        };

        Ok(Some(
            FramedRead::new(file, BytesCodec::new())
                .map_ok(BytesMut::freeze)
                .map_err(ContentStreamError::wrap)
                .boxed(),
        ))
    }

    async fn upload(
        &self,
        file: &File,
        mut content: BoxStream<'_, Result<Bytes, ContentStreamError>>,
    ) -> Result<usize, UploadFileError> {
        let mut size = 0;
        let path = self.path_for_file(file);

        fs::create_dir_all(path.parent().unwrap())
            .await
            .map_err(UploadFileError::wrap)?;

        let mut f = fs::File::create(path)
            .await
            .map_err(UploadFileError::wrap)?;

        while let Some(mut chunk) = content.try_next().await.map_err(UploadFileError::wrap)? {
            while chunk.has_remaining() {
                size += f
                    .write_buf(&mut chunk)
                    .await
                    .map_err(UploadFileError::wrap)?;
            }
        }

        f.flush().await.map_err(UploadFileError::wrap)?;

        Ok(size)
    }
}

#[cfg(test)]
mod tests {

    use crate::file::fixtures::file;
    use assert2::check;
    use futures::Stream;
    use rstest::rstest;

    use super::*;

    fn content(body: impl Into<Bytes>) -> impl Stream<Item = Result<Bytes, ContentStreamError>> {
        futures::stream::iter([Ok(body.into())])
    }

    #[rstest]
    #[tokio::test]
    async fn it_reads_from_a_file(file: File) {
        let root_dir = tempfile::tempdir().unwrap();

        let contents = FsFileContents::new(root_dir.path());

        let path = contents.path_for_file(&file);

        tokio::fs::create_dir_all(path.parent().unwrap())
            .await
            .unwrap();

        tokio::fs::write(&path, b"hello world").await.unwrap();

        let body = contents
            .download(file.owner_id, &file.name)
            .await
            .unwrap()
            .unwrap();

        let body: BytesMut = body.try_collect().await.unwrap();

        check!(body == "hello world");
    }

    #[rstest]
    #[tokio::test]
    async fn it_writes_to_a_file(file: File) {
        let root_dir = tempfile::tempdir().unwrap();

        let contents = FsFileContents::new(root_dir.path());

        let content = content("hello world").boxed();

        let size = contents.upload(&file, content).await.unwrap();

        let path = contents.path_for_file(&file);

        let read = tokio::fs::read_to_string(path).await.unwrap();

        check!(read == "hello world");
        check!(size == "hello world".len());
    }
}
