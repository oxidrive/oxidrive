use axum::Router;

use crate::state::AppState;

mod auth;
mod files;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/files", files::routes())
}
