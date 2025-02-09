use utoipa_axum::{router::OpenApiRouter, routes};

use crate::state::AppState;

mod password;

mod get;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get::handler))
        .nest("/password", password::routes())
}
