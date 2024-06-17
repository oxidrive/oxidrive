use crate::{
    models::{session_request::SessionRequest, Session},
    ApiErrorFromResponse, ApiResult, Client,
};

pub struct SessionService {
    client: Client,
}

impl SessionService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: SessionRequest) -> ApiResult<Session> {
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
        models::{credentials, Credentials, Error, Session},
        tests::{json, json_val},
        ApiError, Oxidrive,
    };
    use assert2::{check, let_assert};
    use chrono::Utc;
    use mockito::Matcher;

    #[tokio::test]
    async fn it_creates_a_new_session_with_username_and_password() {
        let request = SessionRequest {
            credentials: Credentials {
                kind: credentials::Kind::Password,
                username: "test".into(),
                password: "test".into(),
            },
        };

        let expected = Session {
            token: "token".into(),
            expires_at: Utc::now().to_rfc3339(),
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
            credentials: Credentials {
                kind: credentials::Kind::Password,
                username: "test".into(),
                password: "test".into(),
            },
        };

        let expected = Error {
            error: "authentication_failed".into(),
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
