use create::PersonalAccessTokenCreated;
use oxidrive_accounts::pat::PersonalAccessToken;
use serde::Serialize;
use time::OffsetDateTime;
use utoipa::{OpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::state::AppState;

mod create;

#[derive(OpenApi)]
#[openapi(components(
    schemas(PersonalAccessTokenData),
    responses(PersonalAccessTokenCreated)
))]
pub struct PatsApi;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(create::handler))
}

#[derive(Debug, Serialize, ToSchema)]
struct PersonalAccessTokenData {
    id: Uuid,
    #[serde(with = "time::serde::rfc3339::option")]
    expires_at: Option<OffsetDateTime>,
}

impl From<PersonalAccessToken> for PersonalAccessTokenData {
    fn from(pat: PersonalAccessToken) -> Self {
        Self {
            id: pat.id.as_uuid(),
            expires_at: pat.expires_at,
        }
    }
}
