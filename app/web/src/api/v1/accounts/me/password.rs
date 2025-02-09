use utoipa_axum::{router::OpenApiRouter, routes};

use crate::state::AppState;

mod change;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(change::handler))
}
