use std::borrow::Cow;

use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::{ApiErrorFromResponse, ApiResult, Client};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ErrorKind {
    #[serde(other)]
    UnknownError,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUpload {
    pub filename: String,
    pub content: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadResponse {
    pub ok: bool,
    pub id: String,
}

pub struct FileService {
    client: Client,
    token: Option<String>,
}

impl FileService {
    pub(crate) fn new(client: Client, token: Option<String>) -> Self {
        Self { client, token }
    }

    pub async fn upload(
        &self,
        path: impl Into<Cow<'static, str>>,
        file: FileUpload,
    ) -> ApiResult<UploadResponse, ErrorKind> {
        let file = Part::bytes(file.content).file_name(file.filename);
        let form = Form::new().text("path", path).part("file", file);
        let req = self.client.post("/api/files");

        let req = match self.token.as_ref() {
            Some(token) => req.bearer_auth(token),
            None => req,
        };

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
    use crate::{tests::json, Oxidrive};
    use assert2::check;
    use mockito::Matcher;

    #[tokio::test]
    async fn it_creates_a_new_session_with_username_and_password() {
        env_logger::init();

        let path = "path/to/hello.txt";
        let content = b"hello!".to_vec();
        let filename = "hello.txt".to_string();

        let expected = UploadResponse {
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
}
