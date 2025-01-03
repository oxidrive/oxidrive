use axum::Router;

use crate::state::AppState;

mod auth;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/auth", auth::routes())
}
