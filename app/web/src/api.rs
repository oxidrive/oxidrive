use axum::Router;

use crate::state::AppState;

mod v1;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/v1", v1::routes())
}
