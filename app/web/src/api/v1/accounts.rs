use create::AccountCreated;
use oxidrive_accounts::account::Account;
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::state::AppState;

mod create;

#[derive(OpenApi)]
#[openapi(components(schemas(AccountInfo), responses(AccountCreated),))]
pub struct AccountsApi;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(create::handler))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AccountInfo {
    id: Uuid,
    #[schema(examples("admin", "myuser"))]
    username: String,
}

impl From<Account> for AccountInfo {
    fn from(account: Account) -> Self {
        Self {
            id: account.id.as_uuid(),
            username: account.username,
        }
    }
}
