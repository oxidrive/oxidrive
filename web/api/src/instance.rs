use serde::{Deserialize, Serialize};

use crate::{ApiResult, Client};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StatusResponse {
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SetupRequest {
    pub admin: InitialAdminData,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InitialAdminData {
    pub username: String,
    pub password: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SetupResponse {
    pub ok: bool,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub database: String,
    pub file_storage: String,
    #[serde(rename = "publicURL")]
    pub public_url: String,
    pub setup_completed: bool,
}

pub struct InstanceService {
    client: Client,
}

impl InstanceService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn status(&self) -> ApiResult<StatusResponse> {
        let response = self
            .client
            .get("/api/instance")
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    pub async fn setup(&self, req: SetupRequest) -> ApiResult<SetupResponse> {
        let response = self
            .client
            .post("/api/instance/setup")
            .json(&req)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use assert2::check;
    use mockito::Matcher;
    use serde::Serialize;

    use crate::Oxidrive;

    use super::*;

    #[tokio::test]
    async fn it_fetches_the_instance_status() {
        let expected = StatusResponse {
            status: Status {
                database: "sqlite".into(),
                file_storage: "s3".into(),
                public_url: "https://example.com".into(),
                setup_completed: true,
            },
        };
        let mut server = mockito::Server::new_async().await;

        let mock = server
            .mock("GET", "/api/instance")
            .with_header("content-type", "application/json")
            .with_body(json(&expected))
            .create_async()
            .await;

        let instance = Oxidrive::new(server.url()).instance();
        let response = instance.status().await.unwrap();

        mock.assert_async().await;
        check!(response == expected);
    }

    #[tokio::test]
    async fn it_sets_the_instance_up() {
        let expected = SetupResponse { ok: true };
        let request = SetupRequest {
            admin: InitialAdminData {
                username: "test".into(),
                password: "test".into(),
            },
        };

        let mut server = mockito::Server::new_async().await;

        let mock = server
            .mock("POST", "/api/instance/setup")
            .match_header("content-type", "application/json")
            .match_body(Matcher::Json(json_val(&request)))
            .with_header("content-type", "application/json")
            .with_body(json(&expected))
            .create_async()
            .await;

        let instance = Oxidrive::new(server.url()).instance();
        let response = instance.setup(request).await.unwrap();

        mock.assert_async().await;
        check!(response == expected);
    }

    fn json<T: Serialize>(body: &T) -> Vec<u8> {
        serde_json::to_vec(body).unwrap()
    }

    fn json_val<T: Serialize>(body: &T) -> serde_json::Value {
        serde_json::to_value(body).unwrap()
    }
}
