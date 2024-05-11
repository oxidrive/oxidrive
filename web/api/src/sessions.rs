use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::{ApiErrorFromResponse, ApiResult, Client};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ErrorKind {
    AuthenticationFailed,
    #[serde(other)]
    UnknownError,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SessionRequest {
    pub credentials: Credentials,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Credentials {
    Password { username: String, password: String },
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

pub struct SessionService {
    client: Client,
}

impl SessionService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: SessionRequest) -> ApiResult<Session, ErrorKind> {
        let response = self
            .client
            .post("/api/sessions")
            .json(&request)
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
        tests::{json, json_val},
        ApiError, ErrorResponse, Oxidrive,
    };
    use assert2::{check, let_assert};
    use mockito::Matcher;

    #[tokio::test]
    async fn it_creates_a_new_session_with_username_and_password() {
        let request = SessionRequest {
            credentials: Credentials::Password {
                username: "test".into(),
                password: "test".into(),
            },
        };

        let expected = Session {
            token: "token".into(),
            expires_at: Utc::now(),
        };

        let mut server = mockito::Server::new_async().await;

        let mock = server
            .mock("POST", "/api/sessions")
            .match_header("content-type", "application/json")
            .match_body(Matcher::Json(json_val(&request)))
            .with_header("content-type", "application/json")
            .with_body(json(&expected))
            .create_async()
            .await;

        let sessions = Oxidrive::new(server.url()).sessions();
        let response = sessions.create(request).await.unwrap();

        mock.assert_async().await;
        check!(response == expected);
    }

    #[tokio::test]
    async fn it_handles_a_401_response() {
        let request = SessionRequest {
            credentials: Credentials::Password {
                username: "test".into(),
                password: "test".into(),
            },
        };

        let expected = ErrorResponse {
            error: ErrorKind::AuthenticationFailed,
            message: "authentication failed".into(),
        };

        let mut server = mockito::Server::new_async().await;

        let mock = server
            .mock("POST", "/api/sessions")
            .match_header("content-type", "application/json")
            .match_body(Matcher::Json(json_val(&request)))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(json(&expected))
            .create_async()
            .await;

        let sessions = Oxidrive::new(server.url()).sessions();
        let error = sessions.create(request).await.unwrap_err();

        mock.assert_async().await;

        let_assert!(ApiError::Api(error) = error);
        check!(error == expected);
    }
}
