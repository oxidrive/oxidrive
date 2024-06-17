use crate::{
    models::{InstanceSetupRequest, InstanceSetupResponse, InstanceStatus},
    ApiErrorFromResponse, ApiResult, Client,
};

pub struct InstanceService {
    client: Client,
}

impl InstanceService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn status(&self) -> ApiResult<InstanceStatus> {
        let response = self
            .client
            .get("/api/instance")
            .send()
            .await?
            .check_error_response()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    pub async fn setup(&self, req: InstanceSetupRequest) -> ApiResult<InstanceSetupResponse> {
        let response = self
            .client
            .post("/api/instance/setup")
            .json(&req)
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
        models::{Database, FileStorage, InstanceSetupRequestAdmin, InstanceStatusStatus},
        tests::{json, json_val},
        Oxidrive,
    };
    use assert2::check;
    use mockito::Matcher;

    #[tokio::test]
    async fn it_fetches_the_instance_status() {
        let expected = InstanceStatus {
            status: InstanceStatusStatus {
                database: Database::Sqlite,
                file_storage: FileStorage::S3,
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
        let expected = InstanceSetupResponse { ok: true };
        let request = InstanceSetupRequest {
            admin: InstanceSetupRequestAdmin {
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
}
