use axum::Router;

use crate::{Config, state::AppState};

mod session;

pub fn routes(cfg: &Config) -> Router<AppState> {
    Router::new()
        .nest("/session", session::routes())
        .route_layer(cfg.csrf())
}
