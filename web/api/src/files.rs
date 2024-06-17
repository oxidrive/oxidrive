use std::borrow::Cow;

use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};

use crate::{
    models::{file_list::FileList, file_upload_response::FileUploadResponse},
    ApiErrorFromResponse, ApiResult, Client,
};

#[derive(Clone, Debug, Default, Serialize)]
pub struct ListFilesParams {
    pub first: Option<usize>,
    pub after: Option<String>,
    pub prefix: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUpload {
    pub filename: String,
    pub content: Vec<u8>,
}

pub struct FileService {
    client: Client,
}

impl FileService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn list(&self, params: ListFilesParams) -> ApiResult<FileList> {
        let response = self
            .client
            .get("/api/files")
            .query(&params)
            .send()
            .await?
            .check_error_response()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    pub async fn upload(
        &self,
        path: impl Into<Cow<'static, str>>,
        file: FileUpload,
    ) -> ApiResult<FileUploadResponse> {
        let file = Part::bytes(file.content).file_name(file.filename);
        let form = Form::new().text("path", path).part("file", file);
        let req = self.client.post("/api/files");

        let response = req
            .multipart(form)
            .send()
            .await?
            .check_error_response()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::file::{self, File},
        tests::json,
        Oxidrive,
    };
    use assert2::check;
    use mockito::Matcher;
    use uuid::Uuid;

    #[tokio::test]
    async fn it_uploads_a_file() {
        env_logger::init();

        let path = "path/to/hello.txt";
        let content = b"hello!".to_vec();
        let filename = "hello.txt".to_string();

        let expected = FileUploadResponse {
            ok: true,
            id: "test".into(),
        };

        let mut server = mockito::Server::new_async().await;

        let mock = server
            .mock("POST", "/api/files")
            .match_header(
                "content-type",
                Matcher::Regex("^multipart/form-data".into()),
            )
            .with_header("content-type", "application/json")
            .with_body(json(&expected))
            .create_async()
            .await;

        let files = Oxidrive::new(server.url()).files();
        let response = files
            .upload(path, FileUpload { content, filename })
            .await
            .unwrap();

        mock.assert_async().await;
        check!(response == expected);
    }

    #[tokio::test]
    async fn it_lists_all_files_with_a_prefix() {
        env_logger::init();

        let expected = FileList {
            count: 1,
            total: 42,
            next: Some("next".into()),
            items: vec![File {
                id: Uuid::new_v4(),
                content_type: "text/plain".into(),
                r#type: file::Type::File,
                path: "/path/to/file.txt".into(),
                name: "file.txt".into(),
                size: 42,
            }],
        };

        let mut server = mockito::Server::new_async().await;

        let mock = server
            .mock("GET", "/api/files")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("first".into(), "42".into()),
                Matcher::UrlEncoded("after".into(), "test".into()),
                Matcher::UrlEncoded("prefix".into(), "/path/to".into()),
            ]))
            .with_header("content-type", "application/json")
            .with_body(json(&expected))
            .create_async()
            .await;

        let files = Oxidrive::new(server.url()).files();
        let response = files
            .list(ListFilesParams {
                first: Some(42),
                after: Some("test".into()),
                prefix: Some("/path/to".into()),
            })
            .await
            .unwrap();

        mock.assert_async().await;
        check!(response == expected);
    }

    #[tokio::test]
    async fn it_lists_all_files_using_default_params() {
        env_logger::init();

        let expected = FileList {
            count: 1,
            total: 42,
            next: Some("next".into()),
            items: vec![File {
                id: Uuid::new_v4(),
                content_type: "text/plain".into(),
                r#type: file::Type::File,
                path: "/path/to/file.txt".into(),
                name: "file.txt".into(),
                size: 42,
            }],
        };

        let mut server = mockito::Server::new_async().await;

        let mock = server
            .mock("GET", "/api/files")
            .match_query(Matcher::Missing)
            .with_header("content-type", "application/json")
            .with_body(json(&expected))
            .create_async()
            .await;

        let files = Oxidrive::new(server.url()).files();
        let response = files.list(ListFilesParams::default()).await.unwrap();

        mock.assert_async().await;
        check!(response == expected);
    }
}
