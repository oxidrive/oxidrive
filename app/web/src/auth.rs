use axum::Router;

use crate::{state::AppState, Config};

mod session;

pub fn routes(cfg: &Config) -> Router<AppState> {
    Router::new()
        .nest("/session", session::routes())
        .route_layer(cfg.csrf())
}
